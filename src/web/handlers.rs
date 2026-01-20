use actix_web::{web, HttpResponse};

use crate::api::helpers::parse_note_id;
use crate::errors::ApiError;
use crate::related::{fetch_chain, fetch_related};
use crate::state::AppState;
use crate::web::note::note_html;
use crate::web::render::{home_html, signin_html};
use crate::web::templates;

const FAVICON: &[u8] = include_bytes!("assets/icon_256.ico");
const CACHE_STATIC: &str = "public, max-age=31536000, immutable";

pub async fn favicon() -> HttpResponse {
    HttpResponse::Ok()
        .insert_header(("Cache-Control", CACHE_STATIC))
        .content_type("image/vnd.microsoft.icon")
        .body(FAVICON.to_vec())
}

pub async fn home(state: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(home_html(&state.config))
}

pub async fn signin(state: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(signin_html(&state.config))
}

pub async fn terms() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(templates::TERMS)
}

pub async fn privacy() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(templates::PRIVACY)
}

pub async fn guideline() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(templates::GUIDELINE)
}

pub async fn network() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(templates::NETWORK)
}

pub async fn note_page(
    path: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError<serde_json::Value>> {
    let raw = path.into_inner();
    let note_id = parse_note_id(&raw)
        .map_err(|_| ApiError::not_found("not_found", "Route not found"))?;
    let chain = fetch_chain(&state, note_id).await?;
    let related = fetch_related(&state, note_id).await?;
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(note_html(&state.config, &chain, &related.related)))
}
