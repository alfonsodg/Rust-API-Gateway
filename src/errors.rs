use axum::{http::StatusCode, response::{IntoResponse, Response}};
use reqwest::Error;
use std::fmt;
use tracing::{error, warn, info, instrument, Level};

/// Standardized error logging functions
pub fn log_error_with_context(error: &dyn std::fmt::Display, context: &str, error_type: &str) {
    tracing::error!(error = %error, context = %context, error_type = error_type);
    tracing::event!(
        Level::ERROR,
        error_type = error_type,
        error_message = %error,
        context = %context,
        timestamp = chrono::Utc::now().to_rfc3339()
    );
}

pub fn log_warning_with_context(message: &str, context: &str, error_type: &str) {
    tracing::warn!(message = %message, context = %context, error_type = error_type);
    tracing::event!(
        Level::WARN,
        message = %message,
        context = %context,
        error_type = error_type,
        timestamp = chrono::Utc::now().to_rfc3339()
    );
}

pub fn log_info_with_context(message: &str, context: &str, event_type: &str) {
    tracing::info!(message = %message, context = %context, event_type = event_type);
    tracing::event!(
        Level::INFO,
        message = %message,
        context = %context,
        event_type = event_type,
        timestamp = chrono::Utc::now().to_rfc3339()
    );
}

/// Trait for consistent error handling across the application
pub trait ErrorHandler {
    fn log_error(&self, context: &str);
    fn log_warning(&self, context: &str);
    fn get_user_message(&self) -> String;
    fn get_log_level(&self) -> Level;
}

/// Get type name for error logging
fn type_name<T>() -> &'static str {
    std::any::type_name::<T>()
}

/// Error recovery suggestions
pub struct ErrorRecovery {
    pub suggestion: String,
    pub severity: ErrorSeverity,
    pub action_required: bool,
}

#[derive(Debug, Clone)]
pub enum ErrorSeverity {
    Critical,
    High,
    Medium,
    Low,
    Informational,
}

impl ErrorRecovery {
    pub fn critical(suggestion: String) -> Self {
        Self {
            suggestion,
            severity: ErrorSeverity::Critical,
            action_required: true,
        }
    }
    
    pub fn high(suggestion: String) -> Self {
        Self {
            suggestion,
            severity: ErrorSeverity::High,
            action_required: true,
        }
    }
    
    pub fn medium(suggestion: String) -> Self {
        Self {
            suggestion,
            severity: ErrorSeverity::Medium,
            action_required: false,
        }
    }
}

#[derive(Debug)]
pub enum AppError {
    RateLimited,
    ServiceUnavailable,

    // Auth errors
    AuthFailed(String),
    MissingAuthToken,
    InvalidAuthHeader,
    InsufficientPermissions,
    TokenExpired,

    // Proxy errors
    RouteNotFound,
    ProxyError(Error),
    InvalidDestination(String),
    InternalServerError,
    
    // Hot reload errors
    HotReloadError(String),
}

impl ErrorHandler for AppError {
    fn log_error(&self, context: &str) {
        match self {
            AppError::RateLimited => {
                log_warning_with_context("Request rate limited", context, "RateLimited");
            }
            AppError::ServiceUnavailable => {
                log_error_with_context(self, context, "ServiceUnavailable");
            }
            AppError::AuthFailed(reason) => {
                tracing::error!(error = %self, context = %context, reason = %reason, error_type = "AuthFailed");
                tracing::event!(
                    Level::ERROR,
                    error_type = "AuthFailed",
                    error_message = %self,
                    context = %context,
                    reason = %reason,
                    timestamp = chrono::Utc::now().to_rfc3339()
                );
            }
            AppError::MissingAuthToken => {
                log_warning_with_context("Missing authorization token", context, "MissingAuthToken");
            }
            AppError::InvalidAuthHeader => {
                log_warning_with_context("Invalid authorization header", context, "InvalidAuthHeader");
            }
            AppError::InsufficientPermissions => {
                log_warning_with_context("Insufficient permissions", context, "InsufficientPermissions");
            }
            AppError::TokenExpired => {
                log_warning_with_context("Token expired", context, "TokenExpired");
            }
            AppError::RouteNotFound => {
                log_warning_with_context("Route not found", context, "RouteNotFound");
            }
            AppError::ProxyError(_) => {
                log_error_with_context(self, context, "ProxyError");
            }
            AppError::InvalidDestination(url) => {
                tracing::error!(error = %self, context = %context, url = %url, error_type = "InvalidDestination");
                tracing::event!(
                    Level::ERROR,
                    error_type = "InvalidDestination",
                    error_message = %self,
                    context = %context,
                    url = %url,
                    timestamp = chrono::Utc::now().to_rfc3339()
                );
            }
            AppError::InternalServerError => {
                log_error_with_context(self, context, "InternalServerError");
            }
            AppError::HotReloadError(msg) => {
                tracing::error!(error = %self, context = %context, message = %msg, error_type = "HotReloadError");
                tracing::event!(
                    Level::ERROR,
                    error_type = "HotReloadError",
                    error_message = %self,
                    context = %context,
                    message = %msg,
                    timestamp = chrono::Utc::now().to_rfc3339()
                );
            }
        }
    }

    fn log_warning(&self, context: &str) {
        match self {
            AppError::RateLimited | AppError::MissingAuthToken | AppError::InvalidAuthHeader 
            | AppError::InsufficientPermissions | AppError::TokenExpired | AppError::RouteNotFound => {
                log_warning_with_context(&self.to_string(), context, type_name::<Self>());
            }
            _ => {
                self.log_error(context);
            }
        }
    }

    fn get_user_message(&self) -> String {
        self.to_string()
    }

    fn get_log_level(&self) -> Level {
        match self {
            AppError::RateLimited | AppError::MissingAuthToken | AppError::InvalidAuthHeader 
            | AppError::InsufficientPermissions | AppError::TokenExpired | AppError::RouteNotFound => Level::WARN,
            _ => Level::ERROR,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::RateLimited => {
                self.log_warning("HTTP response generation");
                (
                    StatusCode::TOO_MANY_REQUESTS,
                    "Too many requests".to_string(),
                )
            }
            AppError::AuthFailed(ref reason) => {
                self.log_error("HTTP response generation");
                (StatusCode::UNAUTHORIZED, format!("Authentication failed: {}", reason))
            }
            AppError::MissingAuthToken => {
                self.log_warning("HTTP response generation");
                (StatusCode::UNAUTHORIZED, "Missing 'Authorization' header".to_string())
            }
            AppError::InvalidAuthHeader => {
                self.log_warning("HTTP response generation");
                (StatusCode::UNAUTHORIZED, "Invalid 'Authorization' header format. Expected 'Bearer <token>'.".to_string())
            }
            AppError::InsufficientPermissions => {
                self.log_warning("HTTP response generation");
                (StatusCode::FORBIDDEN, "You do not have permission to access this resource.".to_string())
            }
            AppError::TokenExpired => {
                self.log_warning("HTTP response generation");
                (StatusCode::UNAUTHORIZED, "Token has expired".to_string())
            } 
            AppError::RouteNotFound => {
                self.log_warning("HTTP response generation");
                (StatusCode::NOT_FOUND, "Route not found".to_string())
            }
            AppError::ProxyError(_) => {
                self.log_error("HTTP response generation");
                (StatusCode::BAD_GATEWAY, "Error proxying request".to_string())
            }
            AppError::InvalidDestination(ref _url) => {
                self.log_error("HTTP response generation");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Invalid gateway configuration".to_string(),
                )
            }
            AppError::InternalServerError => {
                self.log_error("HTTP response generation");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An internal server error occurred".to_string(),
                )
            }
            AppError::ServiceUnavailable => {
                self.log_error("HTTP response generation");
                (
                    StatusCode::SERVICE_UNAVAILABLE,
                    "Service Unavailable".to_string()
                )
            }
            AppError::HotReloadError(ref _msg) => {
                self.log_error("HTTP response generation");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Configuration reload failed".to_string(),
                )
            }
        };

        (status, error_message).into_response()
    }
}

impl From<reqwest::Error> for AppError {
    fn from(error: reqwest::Error) -> Self {
        AppError::ProxyError(error)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::RateLimited => write!(f, "Rate limited"),
            AppError::ServiceUnavailable => write!(f, "Service unavailable"),
            AppError::AuthFailed(reason) => write!(f, "Authentication failed: {}", reason),
            AppError::MissingAuthToken => write!(f, "Missing authorization token"),
            AppError::InvalidAuthHeader => write!(f, "Invalid authorization header"),
            AppError::InsufficientPermissions => write!(f, "Insufficient permissions"),
            AppError::TokenExpired => write!(f, "Token expired"),
            AppError::RouteNotFound => write!(f, "Route not found"),
            AppError::ProxyError(_) => write!(f, "Proxy error"),
            AppError::InvalidDestination(url) => write!(f, "Invalid destination: {}", url),
            AppError::InternalServerError => write!(f, "Internal server error"),
            AppError::HotReloadError(msg) => write!(f, "Hot reload error: {}", msg),
        }
    }
}