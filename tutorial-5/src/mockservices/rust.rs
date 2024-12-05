use axum::response::IntoResponse;

use super::*;

#[derive(Clone)]
pub struct RustService {
    name: String,
}

impl RustService {
    pub fn new() -> Self {
        RustService {
            name: "Rust".to_string(),
        }
    }

    pub async fn hello(self) -> impl IntoResponse {
        let message = format!("Hello {}", self.name);
        message
    }
}
