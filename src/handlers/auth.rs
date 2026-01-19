use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;

use crate::auth::{google, sessions};
use crate::errors::ApiError;
use crate::handlers::helpers::{parse_json, require_user};
use crate::state::AppState;

#[derive(Deserialize)]
struct GoogleRequest {
    id_token: String,
}

pub async fn post_google(
    state: web::Data<AppState>,
    body: web::Bytes,
) -> Result<HttpResponse, ApiError<serde_json::Value>> {
    let payload: GoogleRequest = parse_json(&body)?;
    let claims = google::validate_id_token(&payload.id_token, &state.config.google_client_id)
        .await
        .map_err(|_| ApiError::unauthorized("token_invalid", "Token invalid"))?;

    let user = state
        .storage
        .find_or_create_user(&claims.sub, &claims.email)
        .await
        .map_err(|_| ApiError::internal())?;

    let token = sessions::generate_token();
    let expires_at = sessions::expires_at(state.config.session_ttl_secs);
    state
        .storage
        .create_session(user.user_id, &token, expires_at)
        .await
        .map_err(|_| ApiError::internal())?;

    let payload = serde_json::json!({
        "token": token,
        "user": user.profile(),
    });
    Ok(HttpResponse::Ok().json(payload))
}

pub async fn get_me(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError<serde_json::Value>> {
    let user = require_user(&req, &state).await?;
    let payload = serde_json::json!({"user": user.profile()});
    Ok(HttpResponse::Ok().json(payload))
}
