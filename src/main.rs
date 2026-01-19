mod api;
mod auth;
mod config;
mod domain;
mod errors;
mod http;
mod related;
mod state;
mod storage;
mod urls;
mod web;

use crate::config::Config;
use crate::http::server::run_server;
use crate::storage::Storage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;
    let storage = Storage::connect(&config).await?;

    if config.run_migrations {
        storage.run_migrations(&config.migrations_path).await?;
    }

    run_server(config, storage).await
}
