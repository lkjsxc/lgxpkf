use serde::Deserialize;

use crate::auth::{google, sessions};
use crate::errors::ApiError;
use crate::http::parser::Request;
use crate::http::response::Response;
use crate::http::router::AppState;

#[derive(Deserialize)]
struct GoogleRequest {
    id_token: String,
}

pub async fn post_google(
    req: Request,
    state: AppState,
) -> Result<Response, ApiError<serde_json::Value>> {
    let body: GoogleRequest = parse_json(&req.body)?;
    let claims = google::validate_id_token(&body.id_token, &state.config.google_client_id)
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
    let json = serde_json::to_vec(&payload).unwrap_or_else(|_| b"{}".to_vec());
    Ok(Response::json(200, json))
}

pub async fn get_me(
    req: Request,
    state: AppState,
) -> Result<Response, ApiError<serde_json::Value>> {
    let token = bearer_token(&req).ok_or_else(|| {
        ApiError::unauthorized("unauthorized", "Missing authorization token")
    })?;

    let user = state
        .storage
        .get_session_user(&token)
        .await
        .map_err(|_| ApiError::internal())?;

    let user = user.ok_or_else(|| ApiError::unauthorized("unauthorized", "Invalid session"))?;
    let payload = serde_json::json!({"user": user.profile()});
    let json = serde_json::to_vec(&payload).unwrap_or_else(|_| b"{}".to_vec());
    Ok(Response::json(200, json))
}

fn bearer_token(req: &Request) -> Option<String> {
    let header = req.headers.get("authorization")?;
    let mut parts = header.split_whitespace();
    match (parts.next(), parts.next()) {
        (Some(scheme), Some(token)) if scheme.eq_ignore_ascii_case("bearer") => {
            Some(token.to_string())
        }
        _ => None,
    }
}

fn parse_json<T: serde::de::DeserializeOwned>(
    body: &[u8],
) -> Result<T, ApiError<serde_json::Value>> {
    serde_json::from_slice(body).map_err(|_| {
        ApiError::bad_request("invalid_json", "Invalid JSON body", None)
    })
}
