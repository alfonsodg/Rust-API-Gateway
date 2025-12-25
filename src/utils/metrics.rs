//! Custom metrics support for business observability.

use std::sync::Arc;
use axum::extract::State;
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::state::AppState;

/// Custom metrics counters
pub struct CustomMetrics {
    pub requests_total: AtomicU64,
    pub requests_success: AtomicU64,
    pub requests_error: AtomicU64,
    pub cache_hits: AtomicU64,
    pub cache_misses: AtomicU64,
    pub rate_limited: AtomicU64,
    pub circuit_breaker_open: AtomicU64,
    pub websocket_connections: AtomicU64,
}

impl CustomMetrics {
    pub fn new() -> Self {
        Self {
            requests_total: AtomicU64::new(0),
            requests_success: AtomicU64::new(0),
            requests_error: AtomicU64::new(0),
            cache_hits: AtomicU64::new(0),
            cache_misses: AtomicU64::new(0),
            rate_limited: AtomicU64::new(0),
            circuit_breaker_open: AtomicU64::new(0),
            websocket_connections: AtomicU64::new(0),
        }
    }

    pub fn inc_requests_total(&self) { self.requests_total.fetch_add(1, Ordering::Relaxed); }
    pub fn inc_requests_success(&self) { self.requests_success.fetch_add(1, Ordering::Relaxed); }
    pub fn inc_requests_error(&self) { self.requests_error.fetch_add(1, Ordering::Relaxed); }
    pub fn inc_cache_hits(&self) { self.cache_hits.fetch_add(1, Ordering::Relaxed); }
    pub fn inc_cache_misses(&self) { self.cache_misses.fetch_add(1, Ordering::Relaxed); }
    pub fn inc_rate_limited(&self) { self.rate_limited.fetch_add(1, Ordering::Relaxed); }
    pub fn inc_circuit_breaker_open(&self) { self.circuit_breaker_open.fetch_add(1, Ordering::Relaxed); }
    pub fn inc_websocket_connections(&self) { self.websocket_connections.fetch_add(1, Ordering::Relaxed); }
    pub fn dec_websocket_connections(&self) { self.websocket_connections.fetch_sub(1, Ordering::Relaxed); }

    /// Render custom metrics in Prometheus format
    pub fn render(&self) -> String {
        format!(
            "# HELP gateway_requests_total Total requests\n\
             # TYPE gateway_requests_total counter\n\
             gateway_requests_total {}\n\
             # HELP gateway_requests_success Successful requests\n\
             # TYPE gateway_requests_success counter\n\
             gateway_requests_success {}\n\
             # HELP gateway_requests_error Error requests\n\
             # TYPE gateway_requests_error counter\n\
             gateway_requests_error {}\n\
             # HELP gateway_cache_hits Cache hits\n\
             # TYPE gateway_cache_hits counter\n\
             gateway_cache_hits {}\n\
             # HELP gateway_cache_misses Cache misses\n\
             # TYPE gateway_cache_misses counter\n\
             gateway_cache_misses {}\n\
             # HELP gateway_rate_limited Rate limited requests\n\
             # TYPE gateway_rate_limited counter\n\
             gateway_rate_limited {}\n\
             # HELP gateway_circuit_breaker_open Circuit breaker open events\n\
             # TYPE gateway_circuit_breaker_open counter\n\
             gateway_circuit_breaker_open {}\n\
             # HELP gateway_websocket_connections Active WebSocket connections\n\
             # TYPE gateway_websocket_connections gauge\n\
             gateway_websocket_connections {}\n",
            self.requests_total.load(Ordering::Relaxed),
            self.requests_success.load(Ordering::Relaxed),
            self.requests_error.load(Ordering::Relaxed),
            self.cache_hits.load(Ordering::Relaxed),
            self.cache_misses.load(Ordering::Relaxed),
            self.rate_limited.load(Ordering::Relaxed),
            self.circuit_breaker_open.load(Ordering::Relaxed),
            self.websocket_connections.load(Ordering::Relaxed),
        )
    }
}

/// Global custom metrics instance
pub static CUSTOM_METRICS: Lazy<CustomMetrics> = Lazy::new(CustomMetrics::new);

/// Metrics handler combining Prometheus and custom metrics
pub async fn metrics_handler(state: State<Arc<AppState>>) -> String {
    let mut output = String::new();
    
    // Add Prometheus metrics if available
    if let Some(handle) = state.prometheus_handle.as_ref() {
        output.push_str(&handle.render());
        output.push('\n');
    }
    
    // Add custom metrics
    output.push_str(&CUSTOM_METRICS.render());
    
    output
}
