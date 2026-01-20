use actix_web::http::header;
use actix_web::HttpRequest;
use serde::de::DeserializeOwned;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;
use url::form_urlencoded;
use uuid::Uuid;

use crate::errors::ApiError;
use crate::state::AppState;
use crate::domain::NoteId;
use crate::urls::base32::{decode_id, is_base32_url};

pub fn parse_json<T: DeserializeOwned>(body: &[u8]) -> Result<T, ApiError<serde_json::Value>> {
    serde_json::from_slice(body)
        .map_err(|_| ApiError::bad_request("invalid_json", "Invalid JSON body", None))
}

pub fn parse_query(req: &HttpRequest) -> Vec<(String, String)> {
    let query = req.query_string();
    if query.is_empty() {
        return Vec::new();
    }
    form_urlencoded::parse(query.as_bytes())
        .into_owned()
        .collect()
}

pub fn parse_query_param<'a>(params: &'a [(String, String)], key: &str) -> Option<&'a str> {
    params.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str())
}

pub fn parse_time_param(
    params: &[(String, String)],
    key: &str,
) -> Result<Option<OffsetDateTime>, ApiError<serde_json::Value>> {
    if let Some(value) = parse_query_param(params, key) {
        let parsed = OffsetDateTime::parse(value, &Rfc3339)
            .map_err(|_| ApiError::bad_request("invalid_timestamp", "Invalid timestamp", None))?;
        Ok(Some(parsed))
    } else {
        Ok(None)
    }
}

pub fn parse_limit_param(
    params: &[(String, String)],
    key: &str,
    default: usize,
    max: usize,
) -> Result<usize, ApiError<serde_json::Value>> {
    match parse_query_param(params, key) {
        Some(value) => {
            let parsed = value
                .parse::<usize>()
                .map_err(|_| ApiError::bad_request("invalid_limit", "Invalid limit", None))?;
            if parsed == 0 {
                return Err(ApiError::bad_request(
                    "invalid_limit",
                    "Limit must be positive",
                    None,
                ));
            }
            if parsed > max {
                return Err(ApiError::unprocessable(
                    "limit_too_large",
                    "Limit exceeds maximum",
                    None,
                ));
            }
            Ok(parsed)
        }
        None => Ok(default),
    }
}

pub fn parse_note_id(value: &str) -> Result<NoteId, ApiError<serde_json::Value>> {
    if value.is_empty() || !is_base32_url(value) {
        return Err(ApiError::bad_request("invalid_id", "Invalid note id", None));
    }
    decode_id(value)
        .map(NoteId::from_bytes)
        .ok_or_else(|| ApiError::bad_request("invalid_id", "Invalid note id", None))
}

pub fn parse_uuid(
    value: &str,
    code: &'static str,
    message: &'static str,
) -> Result<Uuid, ApiError<serde_json::Value>> {
    value
        .parse::<Uuid>()
        .map_err(|_| ApiError::bad_request(code, message, None))
}

pub async fn require_user(
    req: &HttpRequest,
    state: &AppState,
) -> Result<crate::domain::User, ApiError<serde_json::Value>> {
    let token = bearer_token(req).ok_or_else(|| {
        ApiError::unauthorized("unauthorized", "Missing authorization token")
    })?;

    let user = state
        .storage
        .get_session_user(&token)
        .await
        .map_err(|_| ApiError::internal())?;

    user.ok_or_else(|| ApiError::unauthorized("unauthorized", "Invalid session"))
}

fn bearer_token(req: &HttpRequest) -> Option<String> {
    let header = req.headers().get(header::AUTHORIZATION)?.to_str().ok()?;
    let mut parts = header.split_whitespace();
    match (parts.next(), parts.next()) {
        (Some(scheme), Some(token)) if scheme.eq_ignore_ascii_case("bearer") => {
            Some(token.to_string())
        }
        _ => None,
    }
}
