use axum::{
    response::{IntoResponse, Response},
    Json,
};
use chrono::{DateTime, Utc};
use hyper::body::to_bytes;
use reqwest::StatusCode;
use serde_json::json;

#[derive(Clone, Debug, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Unhealthy(Option<String>),
}

pub trait HealthCheckable {
    fn health_status(&self) -> HealthStatus;
    fn get_since_last_update(&self) -> DateTime<Utc>;
}

impl IntoResponse for HealthStatus {
    fn into_response(self) -> Response {
        match self {
            HealthStatus::Healthy => StatusCode::OK.into_response(),
            HealthStatus::Unhealthy(message) => {
                let message = message.unwrap_or_else(|| {
                    "eth-analysis module unhealthy".to_string()
                });
                let body = json!({"message": message});
                (StatusCode::SERVICE_UNAVAILABLE, Json(body)).into_response()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_status() {
        let item = HealthStatus::Healthy;
        let response = item.into_response();
        assert_eq!(response.status(), StatusCode::OK);
        println!("status {:?}", response.status());
        let body_bytes = to_bytes(response.into_body()).await.unwrap();
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
        println!("body {:?}", body_str);
    }

    #[tokio::test]
    async fn test_unhealth_status() {
        let item = HealthStatus::Unhealthy(Some("unhealth status".to_string()));
        let response = item.into_response();
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
        println!("unhealth response: {:?}", response.body());
    }
}
