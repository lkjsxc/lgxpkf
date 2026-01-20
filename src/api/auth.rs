use actix_web::http::header;
use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;
use std::collections::HashMap;
use url::form_urlencoded;

use crate::api::helpers::{parse_json, require_user};
use crate::auth::{google, sessions};
use crate::errors::ApiError;
use crate::state::AppState;
use crate::web as web_views;

const POLICY_VERSION: &str = "2025-02-01";

#[derive(Deserialize)]
struct GoogleRequest {
    id_token: String,
    policy_acceptance: Option<PolicyAcceptance>,
}

#[derive(Deserialize)]
struct PolicyAcceptance {
    accepted: bool,
    version: String,
    agreed_at: Option<String>,
}

#[derive(Deserialize)]
struct RedirectState {
    path: Option<String>,
    policy_acceptance: Option<PolicyAcceptance>,
}

pub async fn post_google(
    body: web::Bytes,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError<serde_json::Value>> {
    let payload: GoogleRequest = parse_json(body.as_ref())?;
    require_policy(payload.policy_acceptance.as_ref())?;
    let (user, token) = issue_session(&payload.id_token, &state).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "token": token,
        "user": user.profile(),
    })))
}

pub async fn get_me(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError<serde_json::Value>> {
    let user = require_user(&req, &state).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "user": user.profile(),
    })))
}

pub async fn post_google_redirect(
    req: HttpRequest,
    body: web::Bytes,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError<serde_json::Value>> {
    let form = parse_form(body.as_ref());
    if !csrf_valid(&form, req.headers().get(header::COOKIE)) {
        return Err(ApiError::unauthorized("csrf_invalid", "Invalid CSRF token"));
    }
    let credential = form
        .get("credential")
        .or_else(|| form.get("id_token"))
        .ok_or_else(|| {
            ApiError::bad_request("missing_token", "Missing Google credential", None)
        })?;
    let (target, acceptance) = parse_state(form.get("state"));
    require_policy(acceptance.as_ref())?;
    let (_user, token) = issue_session(credential, &state).await?;
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(web_views::redirect_html(&token, &target)))
}

async fn issue_session(
    id_token: &str,
    state: &AppState,
) -> Result<(crate::domain::User, String), ApiError<serde_json::Value>> {
    let claims = google::validate_id_token(id_token, &state.config.google_client_id)
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
    Ok((user, token))
}

fn parse_form(body: &[u8]) -> HashMap<String, String> {
    form_urlencoded::parse(body).into_owned().collect()
}

fn csrf_valid(
    form: &HashMap<String, String>,
    cookie_header: Option<&header::HeaderValue>,
) -> bool {
    let Some(token) = form.get("g_csrf_token") else {
        return true;
    };
    let cookie = cookie_header
        .and_then(|header| header.to_str().ok())
        .and_then(|header| parse_cookie(header).get("g_csrf_token").cloned());
    cookie.as_deref() == Some(token.as_str())
}

fn parse_cookie(header: &str) -> HashMap<String, String> {
    header
        .split(';')
        .filter_map(|pair| {
            let mut parts = pair.trim().splitn(2, '=');
            let key = parts.next()?.trim();
            let value = parts.next()?.trim();
            if key.is_empty() {
                None
            } else {
                Some((key.to_string(), value.to_string()))
            }
        })
        .collect()
}

fn sanitize_redirect(state: Option<&str>) -> String {
    let target = state.unwrap_or("/");
    if target.starts_with('/') && !target.starts_with("//") && !target.contains("://") {
        target.to_string()
    } else {
        "/".to_string()
    }
}

fn parse_state(state: Option<&String>) -> (String, Option<PolicyAcceptance>) {
    let raw = state.map(|s| s.as_str()).unwrap_or("/");
    if let Ok(parsed) = serde_json::from_str::<RedirectState>(raw) {
        let path = sanitize_redirect(parsed.path.as_deref());
        return (path, parsed.policy_acceptance);
    }
    (sanitize_redirect(Some(raw)), None)
}

fn require_policy(
    acceptance: Option<&PolicyAcceptance>,
) -> Result<(), ApiError<serde_json::Value>> {
    let acceptance = acceptance.ok_or_else(|| {
        ApiError::unprocessable("policy_required", "Policy acceptance required", None)
    })?;
    if !acceptance.accepted || acceptance.version != POLICY_VERSION {
        return Err(ApiError::unprocessable(
            "policy_required",
            "Policy acceptance required",
            None,
        ));
    }
    Ok(())
}
