pub mod associations;
pub mod auth;
pub mod feed;
pub mod follows;
pub mod helpers;
pub mod notes;
pub mod related;
pub mod site;
pub mod users;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(site::home)));
    cfg.service(web::resource("/auth/google").route(web::post().to(auth::post_google)));
    cfg.service(web::resource("/auth/me").route(web::get().to(auth::get_me)));
    cfg.service(
        web::resource("/notes")
            .route(web::post().to(notes::post_notes))
            .route(web::get().to(notes::get_notes)),
    );
    cfg.service(web::resource("/notes/{id}").route(web::get().to(notes::get_note_by_id)));
    cfg.service(
        web::resource("/notes/{id}/related").route(web::get().to(related::get_related)),
    );
    cfg.service(web::resource("/feed").route(web::get().to(feed::get_feed)));
    cfg.service(
        web::resource("/associations")
            .route(web::post().to(associations::post_associations))
            .route(web::get().to(associations::get_associations)),
    );
    cfg.service(
        web::resource("/follows")
            .route(web::post().to(follows::post_follows))
            .route(web::delete().to(follows::delete_follows))
            .route(web::get().to(follows::get_follows)),
    );
    cfg.service(web::resource("/users/{id}").route(web::get().to(users::get_user_by_id)));
}
