use serde::Serialize;
use url::form_urlencoded;

use crate::api::{account, associations, auth, feed, follows, health, notes, related, users};
use crate::errors::{ApiError, ErrorBody};
use crate::http::parser::Request;
use crate::http::response::Response;
use crate::state::AppState;
use crate::urls::base32::is_base32_url;
use crate::web;

pub async fn route(req: Request, state: AppState) -> Response {
    let method = req.method.as_str();
    let path = req.path.as_str();

    let result = match (method, path) {
        ("GET", "/") => Ok(web::home(&state.config)),
        ("GET", "/favicon.ico") => Ok(web::favicon()),
        ("GET", "/health") => health::get_health(req, state).await,
        ("GET", "/ready") => health::get_ready(req, state).await,
        ("POST", "/auth/google") => auth::post_google(req, state).await,
        ("POST", "/auth/google/redirect") => auth::post_google_redirect(req, state).await,
        ("GET", "/auth/me") => auth::get_me(req, state).await,
        ("POST", "/account/note") => account::post_account_note(req, state).await,
        ("POST", "/notes") => notes::post_notes(req, state).await,
        ("GET", "/notes") => notes::get_notes(req, state).await,
        ("GET", "/feed") => feed::get_feed(req, state).await,
        ("POST", "/associations") => associations::post_associations(req, state).await,
        ("GET", "/associations") => associations::get_associations(req, state).await,
        ("POST", "/follows") => follows::post_follows(req, state).await,
        ("DELETE", "/follows") => follows::delete_follows(req, state).await,
        ("GET", "/follows") => follows::get_follows(req, state).await,
        _ if method == "GET" && path.starts_with("/notes/") && path.ends_with("/related") => {
            related::get_related(req, state).await
        }
        _ if method == "GET" && path.starts_with("/notes/") => {
            notes::get_note_by_id(req, state).await
        }
        _ if method == "GET" && path.starts_with("/users/") => {
            users::get_user_by_id(req, state).await
        }
        _ if method == "GET" => match note_slug(path) {
            Some(note_id) => web::note_page(note_id, state).await,
            None => Err(ApiError::<serde_json::Value>::not_found(
                "not_found",
                "Route not found",
            )),
        },
        _ => Err(ApiError::<serde_json::Value>::not_found(
            "not_found",
            "Route not found",
        )),
    };

    match result {
        Ok(resp) => resp,
        Err(err) => error_response(err),
    }
}

pub fn parse_query(query: Option<&str>) -> Vec<(String, String)> {
    match query {
        Some(q) => form_urlencoded::parse(q.as_bytes())
            .into_owned()
            .collect(),
        None => Vec::new(),
    }
}

fn error_response<T: Serialize>(err: ApiError<T>) -> Response {
    let body = ErrorBody {
        code: err.code,
        message: err.message,
        details: err.details,
    };
    let json = serde_json::to_vec(&body).unwrap_or_else(|_| b"{}".to_vec());
    Response::json(err.status, json)
}

fn note_slug(path: &str) -> Option<&str> {
    let trimmed = path.strip_prefix('/')?;
    if trimmed.is_empty() || trimmed.contains('/') {
        return None;
    }
    if !is_base32_url(trimmed) {
        return None;
    }
    Some(trimmed)
}
