pub mod auth;
pub mod rate_limiter;
pub mod cache;
pub mod request_id;
pub mod circuit_breaker;

use std::sync::Arc;
use crate::{config::RouteConfig, state::AppState};

/// Common helper to get route configuration for a path
pub async fn get_route_config(
    state: &Arc<AppState>,
    path: &str,
) -> Option<Arc<RouteConfig>> {
    let config_guard = state.config.read().await;
    config_guard.find_route_for_path(path)
}