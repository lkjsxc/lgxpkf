use actix_web::{web, HttpResponse};

use crate::api::helpers::parse_note_id;
use crate::errors::ApiError;
use crate::related::fetch_related;
use crate::state::AppState;

pub async fn get_related(
    path: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError<serde_json::Value>> {
    let note_id = parse_note_id(path.as_str())?;
    let response = fetch_related(&state, note_id).await?;
    Ok(HttpResponse::Ok().json(response))
}
