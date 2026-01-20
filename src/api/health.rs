use actix_web::web;
use actix_web::HttpResponse;
use serde::Serialize;

use crate::errors::ApiError;
use crate::state::AppState;

#[derive(Serialize)]
struct HealthStatus {
    status: &'static str,
}

pub async fn get_health(
    _state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError<serde_json::Value>> {
    Ok(HttpResponse::Ok().json(HealthStatus { status: "ok" }))
}

pub async fn get_ready(
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError<serde_json::Value>> {
    state
        .storage
        .healthcheck()
        .await
        .map_err(|_| ApiError::service_unavailable("not_ready", "Database unavailable"))?;

    Ok(HttpResponse::Ok().json(HealthStatus { status: "ready" }))
}
