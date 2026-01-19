mod auth;
mod config;
mod domain;
mod errors;
mod handlers;
mod state;
mod storage;
mod urls;
mod web;

use crate::config::Config;
use crate::handlers::configure;
use crate::state::AppState;
use crate::storage::Storage;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;
    let storage = Storage::connect(&config).await?;

    if config.run_migrations {
        storage.run_migrations(&config.migrations_path).await?;
    }

    let state = AppState { config, storage };
    let bind_addr = state.config.bind_addr.clone();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .configure(configure)
    })
    .bind(bind_addr)?
    .run()
    .await?;

    Ok(())
}
