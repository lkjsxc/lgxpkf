use serde::Serialize;

use crate::errors::ApiError;
use crate::http::parser::Request;
use crate::http::response::Response;
use crate::state::AppState;

#[derive(Serialize)]
struct HealthStatus {
    status: &'static str,
}

pub async fn get_health(
    _req: Request,
    _state: AppState,
) -> Result<Response, ApiError<serde_json::Value>> {
    let body = HealthStatus { status: "ok" };
    let json = serde_json::to_vec(&body).unwrap_or_else(|_| b"{}".to_vec());
    Ok(Response::json(200, json))
}

pub async fn get_ready(
    _req: Request,
    state: AppState,
) -> Result<Response, ApiError<serde_json::Value>> {
    state
        .storage
        .healthcheck()
        .await
        .map_err(|_| ApiError::service_unavailable("not_ready", "Database unavailable"))?;

    let body = HealthStatus { status: "ready" };
    let json = serde_json::to_vec(&body).unwrap_or_else(|_| b"{}".to_vec());
    Ok(Response::json(200, json))
}
