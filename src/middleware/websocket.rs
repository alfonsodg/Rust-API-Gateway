//! WebSocket proxy support for real-time connections.

use std::sync::Arc;
use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, Path, State},
    response::Response,
};
use futures::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use tracing::{error, info};

use crate::{errors::AppError, state::AppState};

/// WebSocket upgrade handler
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> Result<Response, AppError> {
    let config_guard = state.config.read().await;
    let route = config_guard
        .find_route_for_path(&format!("/{}", path))
        .ok_or(AppError::RouteNotFound)?;

    let destination = if !route.destinations.is_empty() {
        route.destinations[0].clone()
    } else if !route.destination.is_empty() {
        route.destination.clone()
    } else {
        return Err(AppError::RouteNotFound);
    };

    // Convert http(s) to ws(s)
    let ws_url = destination
        .replace("http://", "ws://")
        .replace("https://", "wss://");

    drop(config_guard);

    Ok(ws.on_upgrade(move |socket| handle_ws_proxy(socket, ws_url)))
}

/// Proxy WebSocket messages between client and backend
async fn handle_ws_proxy(client_ws: WebSocket, backend_url: String) {
    let backend_conn = match connect_async(&backend_url).await {
        Ok((ws, _)) => ws,
        Err(e) => {
            error!(error = %e, "Failed to connect to backend WebSocket");
            return;
        }
    };

    info!(url = %backend_url, "WebSocket proxy connection established");

    let (mut client_sink, mut client_stream) = client_ws.split();
    let (mut backend_sink, mut backend_stream) = backend_conn.split();

    // Forward client -> backend
    let client_to_backend = async {
        while let Some(Ok(msg)) = client_stream.next().await {
            let backend_msg = match msg {
                Message::Text(t) => tokio_tungstenite::tungstenite::Message::Text(t.to_string().into()),
                Message::Binary(b) => tokio_tungstenite::tungstenite::Message::Binary(b.into()),
                Message::Ping(p) => tokio_tungstenite::tungstenite::Message::Ping(p.into()),
                Message::Pong(p) => tokio_tungstenite::tungstenite::Message::Pong(p.into()),
                Message::Close(_) => return,
            };
            if backend_sink.send(backend_msg).await.is_err() {
                return;
            }
        }
    };

    // Forward backend -> client
    let backend_to_client = async {
        while let Some(Ok(msg)) = backend_stream.next().await {
            let client_msg = match msg {
                tokio_tungstenite::tungstenite::Message::Text(t) => Message::Text(t.to_string().into()),
                tokio_tungstenite::tungstenite::Message::Binary(b) => Message::Binary(b.to_vec().into()),
                tokio_tungstenite::tungstenite::Message::Ping(p) => Message::Ping(p.to_vec().into()),
                tokio_tungstenite::tungstenite::Message::Pong(p) => Message::Pong(p.to_vec().into()),
                tokio_tungstenite::tungstenite::Message::Close(_) => return,
                _ => continue,
            };
            if client_sink.send(client_msg).await.is_err() {
                return;
            }
        }
    };

    tokio::select! {
        _ = client_to_backend => {},
        _ = backend_to_client => {},
    }

    info!("WebSocket proxy connection closed");
}
