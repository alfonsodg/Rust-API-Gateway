//! Centralized logging utilities for consistent application logging.
//!
//! This module provides standardized logging functions that ensure consistent
//! formatting, context inclusion, and log levels across the entire application.

use tracing::{debug, error, info, warn, Level};
use chrono::Utc;

/// Standardized info logging with context
pub fn log_info(message: &str, context: &str, event_type: &str) {
    info!(message = %message, context = %context, event_type = event_type);
    tracing::event!(
        Level::INFO,
        event_type = event_type,
        message = %message,
        context = %context,
        timestamp = Utc::now().to_rfc3339()
    );
}

/// Standardized warning logging with context
pub fn log_warn(message: &str, context: &str, warning_type: &str) {
    warn!(message = %message, context = %context, warning_type = warning_type);
    tracing::event!(
        Level::WARN,
        warning_type = warning_type,
        message = %message,
        context = %context,
        timestamp = Utc::now().to_rfc3339()
    );
}

/// Standardized error logging with context
pub fn log_error(error: &dyn std::fmt::Display, context: &str, error_type: &str) {
    error!(error = %error, context = %context, error_type = error_type);
    tracing::event!(
        Level::ERROR,
        error_type = error_type,
        error_message = %error,
        context = %context,
        timestamp = Utc::now().to_rfc3339()
    );
}

/// Standardized debug logging with context
pub fn log_debug(message: &str, context: &str, debug_type: &str) {
    debug!(message = %message, context = %context, debug_type = debug_type);
}

/// Structured logging for request handling
pub fn log_request(method: &str, uri: &str, client_ip: &str, status_code: u16, response_time_ms: u64) {
    tracing::info!(
        method = %method,
        uri = %uri,
        client_ip = %client_ip,
        status_code = %status_code,
        response_time_ms = %response_time_ms,
        event_type = "request_completed"
    );
}

/// Structured logging for authentication events
pub fn log_auth_event(event: &str, user_id: &str, success: bool, reason: Option<&str>) {
    tracing::info!(
        event = %event,
        user_id = %user_id,
        success = %success,
        reason = reason,
        event_type = "auth_event"
    );
}

/// Structured logging for security events
pub fn log_security_event(event: &str, source_ip: &str, details: &str, severity: &str) {
    tracing::warn!(
        security_event = %event,
        source_ip = %source_ip,
        details = %details,
        severity = %severity,
        event_type = "security_event"
    );
}

/// Structured logging for performance metrics
pub fn log_performance_metric(metric_name: &str, value: f64, unit: &str, context: &str) {
    tracing::debug!(
        metric_name = %metric_name,
        value = %value,
        unit = %unit,
        context = %context,
        event_type = "performance_metric"
    );
}

/// Structured logging for configuration changes
pub fn log_config_change(config_item: &str, old_value: Option<&str>, new_value: &str) {
    tracing::info!(
        config_item = %config_item,
        old_value = old_value,
        new_value = %new_value,
        event_type = "config_change"
    );
}

/// Structured logging for cache operations
pub fn log_cache_operation(operation: &str, key: &str, hit: bool, ttl_seconds: Option<u64>) {
    tracing::debug!(
        cache_operation = %operation,
        cache_key = %key,
        cache_hit = %hit,
        ttl_seconds = ttl_seconds,
        event_type = "cache_operation"
    );
}

/// Structured logging for circuit breaker events
pub fn log_circuit_breaker_event(route: &str, old_state: &str, new_state: &str, reason: &str) {
    tracing::info!(
        route = %route,
        old_state = %old_state,
        new_state = %new_state,
        reason = %reason,
        event_type = "circuit_breaker_state_change"
    );
}

/// Structured logging for rate limiting events
pub fn log_rate_limit_event(client_ip: &str, path: &str, limit_exceeded: bool, current_count: u64) {
    if limit_exceeded {
        tracing::warn!(
            client_ip = %client_ip,
            path = %path,
            limit_exceeded = %limit_exceeded,
            current_count = %current_count,
            event_type = "rate_limit_exceeded"
        );
    } else {
        tracing::debug!(
            client_ip = %client_ip,
            path = %path,
            current_count = %current_count,
            event_type = "rate_limit_check"
        );
    }
}

/// Structured logging for hot reload events
pub fn log_hot_reload_event(event: &str, file_path: &str, success: bool, error_message: Option<&str>) {
    if success {
        tracing::info!(
            reload_event = %event,
            file_path = %file_path,
            success = %success,
            event_type = "hot_reload_success"
        );
    } else {
        tracing::error!(
            reload_event = %event,
            file_path = %file_path,
            success = %success,
            error_message = error_message,
            event_type = "hot_reload_failure"
        );
    }
}

/// Standardized startup logging
pub fn log_startup(component: &str, status: &str, details: Option<&str>) {
    info!(
        component = %component,
        status = %status,
        details = details,
        event_type = "application_startup"
    );
}

/// Standardized shutdown logging
pub fn log_shutdown(component: &str, status: &str, duration_ms: Option<u64>) {
    info!(
        component = %component,
        status = %status,
        duration_ms = duration_ms,
        event_type = "application_shutdown"
    );
}
