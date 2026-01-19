use actix_web::{web, HttpResponse};

use crate::state::AppState;

pub async fn home(state: web::Data<AppState>) -> HttpResponse {
    let body = crate::web::home_html(&state.config);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}
