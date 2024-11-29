use axum::response::IntoResponse;

use super::*;

#[derive(Clone)]
pub struct Web3Service {
    name: String,
}

impl Web3Service {
    pub fn new() -> Self {
        Web3Service {
            name: "Web 3.0".to_owned(),
        }
    }

    pub async fn hello(self) -> impl IntoResponse {
        let message = format!("Hello {}", self.name);
        message
    }
}
