use std::sync::Arc;

use axum::extract::State;

use crate::state::AppState;

pub async fn metrics_handler(state: State<Arc<AppState>>) -> String {
    state.prometheus_handle.as_ref().unwrap().render()
}
