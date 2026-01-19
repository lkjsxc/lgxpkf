use serde::de::DeserializeOwned;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

use crate::errors::ApiError;
use crate::http::parser::Request;
use crate::http::router::AppState;

pub fn parse_json<T: DeserializeOwned>(
    body: &[u8],
) -> Result<T, ApiError<serde_json::Value>> {
    serde_json::from_slice(body).map_err(|_| {
        ApiError::bad_request("invalid_json", "Invalid JSON body", None)
    })
}

pub fn parse_query_param<'a>(
    params: &'a [(String, String)],
    key: &str,
) -> Option<&'a str> {
    params.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str())
}

pub fn parse_time_param(
    params: &[(String, String)],
    key: &str,
) -> Result<Option<OffsetDateTime>, ApiError<serde_json::Value>> {
    if let Some(value) = parse_query_param(params, key) {
        let parsed = OffsetDateTime::parse(value, &Rfc3339).map_err(|_| {
            ApiError::bad_request("invalid_timestamp", "Invalid timestamp", None)
        })?;
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
            let parsed = value.parse::<usize>().map_err(|_| {
                ApiError::bad_request("invalid_limit", "Invalid limit", None)
            })?;
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

pub async fn require_user(
    req: &Request,
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
