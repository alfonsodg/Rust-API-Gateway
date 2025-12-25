// Common test utilities
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

/// Get JWT secret from environment or use test default
pub fn get_test_jwt_secret() -> String {
    std::env::var("JWT_SECRET").unwrap_or_else(|_| "super-secret-jwt-key-for-testing-only".to_string())
}

/// Generate a valid JWT for testing with specified roles
pub fn generate_test_jwt(roles: Vec<&str>) -> String {
    let claims = json!({
        "sub": "test-user",
        "roles": roles,
        "exp": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + 3600,
    });
    let secret = get_test_jwt_secret();
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap()
}
