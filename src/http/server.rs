use actix_files::Files;
use actix_web::middleware::{DefaultHeaders, Logger};
use actix_web::{web, App, HttpServer};

use crate::api::{account, associations, auth, feed, follows, health, notes, related, users};
use crate::config::Config;
use crate::state::AppState;
use crate::storage::Storage;
use crate::web as web_views;

const MAX_BODY_BYTES: usize = 1024 * 1024;
const CACHE_STATIC: &str = "public, max-age=31536000, immutable";

pub async fn run_server(
    config: Config,
    storage: Storage,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let bind_addr = config.bind_addr.clone();
    let state = AppState { config, storage };

    HttpServer::new(move || {
        let payload_config = web::PayloadConfig::new(MAX_BODY_BYTES);
        App::new()
            .app_data(web::Data::new(state.clone()))
            .app_data(payload_config)
            .wrap(Logger::default())
            .service(web::resource("/").route(web::get().to(web_views::home)))
            .service(web::resource("/signin").route(web::get().to(web_views::signin)))
            .service(web::resource("/terms").route(web::get().to(web_views::terms)))
            .service(web::resource("/privacy").route(web::get().to(web_views::privacy)))
            .service(web::resource("/guideline").route(web::get().to(web_views::guideline)))
            .service(web::resource("/network").route(web::get().to(web_views::network)))
            .service(web::resource("/favicon.ico").route(web::get().to(web_views::favicon)))
            .service(web::resource("/health").route(web::get().to(health::get_health)))
            .service(web::resource("/ready").route(web::get().to(health::get_ready)))
            .service(web::resource("/auth/google").route(web::post().to(auth::post_google)))
            .service(
                web::resource("/auth/google/redirect")
                    .route(web::post().to(auth::post_google_redirect)),
            )
            .service(web::resource("/auth/me").route(web::get().to(auth::get_me)))
            .service(web::resource("/account/note").route(web::post().to(account::post_account_note)))
            .service(
                web::resource("/notes")
                    .route(web::post().to(notes::post_notes))
                    .route(web::get().to(notes::get_notes)),
            )
            .service(web::resource("/notes/{id}/versions").route(web::post().to(notes::post_note_version)))
            .service(web::resource("/notes/random").route(web::get().to(notes::get_random_notes)))
            .service(web::resource("/feed").route(web::get().to(feed::get_feed)))
            .service(
                web::resource("/associations")
                    .route(web::post().to(associations::post_associations))
                    .route(web::get().to(associations::get_associations)),
            )
            .service(
                web::resource("/follows")
                    .route(web::post().to(follows::post_follows))
                    .route(web::delete().to(follows::delete_follows))
                    .route(web::get().to(follows::get_follows)),
            )
            .service(web::resource("/notes/{id}/related").route(web::get().to(related::get_related)))
            .service(web::resource("/notes/{id}").route(web::get().to(notes::get_note_by_id)))
            .service(web::resource("/users/{id}").route(web::get().to(users::get_user_by_id)))
            .service(web::resource("/{id}").route(web::get().to(web_views::note_page)))
            .service(
                web::scope("/assets")
                    .wrap(DefaultHeaders::new().add(("Cache-Control", CACHE_STATIC)))
                    .service(Files::new("", "public/assets").use_etag(true).use_last_modified(true)),
            )
    })
    .bind(bind_addr)?
    .run()
    .await?;
    Ok(())
}
