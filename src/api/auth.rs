use serde::Deserialize;
use std::collections::HashMap;
use url::form_urlencoded;

use crate::api::helpers::{parse_json, require_user};
use crate::auth::{google, sessions};
use crate::errors::ApiError;
use crate::http::parser::Request;
use crate::http::response::Response;
use crate::state::AppState;

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
    req: Request,
    state: AppState,
) -> Result<Response, ApiError<serde_json::Value>> {
    let body: GoogleRequest = parse_json(&req.body)?;
    require_policy(body.policy_acceptance.as_ref())?;
    let (user, token) = issue_session(&body.id_token, &state).await?;

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
    let user = require_user(&req, &state).await?;
    let payload = serde_json::json!({"user": user.profile()});
    let json = serde_json::to_vec(&payload).unwrap_or_else(|_| b"{}".to_vec());
    Ok(Response::json(200, json))
}

pub async fn post_google_redirect(
    req: Request,
    state: AppState,
) -> Result<Response, ApiError<serde_json::Value>> {
    let form = parse_form(&req.body);
    if !csrf_valid(&form, req.headers.get("cookie")) {
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
    Ok(Response::html(redirect_html(&token, &target)))
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
    cookie_header: Option<&String>,
) -> bool {
    let Some(token) = form.get("g_csrf_token") else { return true };
    let cookie = cookie_header
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
            if key.is_empty() { None } else { Some((key.to_string(), value.to_string())) }
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

fn redirect_html(token: &str, target: &str) -> String {
    let token_json = serde_json::to_string(token).unwrap_or_else(|_| "\"\"".to_string());
    let target_json = serde_json::to_string(target).unwrap_or_else(|_| "\"/\"".to_string());
    format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><meta name=\"theme-color\" content=\"#0b111c\"><title>lgxpkf</title><style>html,body{{margin:0;background:#0b111c;color:#e7eef8;font-family:sans-serif}}</style></head><body><script>localStorage.setItem(\"lgxpkf.session\", {token_json});window.location.replace({target_json});</script></body></html>"
    )
}
