use super::*;

use axum::response::IntoResponse;

#[derive(Clone)]
pub struct WorldService {
    name: String,
}

impl WorldService {
    pub fn new() -> Self {
        WorldService {
            name: "World Service".to_string(),
        }
    }
    pub async fn hello(self) -> impl IntoResponse {
        let message = format!("Hello {}", self.name);
        message.into_response()
    }
}
