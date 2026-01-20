use std::env;

#[derive(Clone)]
pub struct Config {
    pub bind_addr: String,
    pub database_url: String,
    pub google_client_id: String,
    pub public_base_url: String,
    pub session_ttl_secs: i64,
    pub run_migrations: bool,
    pub migrations_path: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
        let database_url = env::var("DATABASE_URL")?;
        let google_client_id = env::var("GOOGLE_CLIENT_ID")?;
        let public_base_url = env::var("PUBLIC_BASE_URL")?;
        let public_base_url = normalize_base_url(public_base_url)?;
        let session_ttl_secs = env::var("SESSION_TTL_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(3600);
        let run_migrations = env::var("RUN_MIGRATIONS")
            .ok()
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let migrations_path = env::var("MIGRATIONS_PATH")
            .unwrap_or_else(|_| "db/migrations".to_string());

        Ok(Self {
            bind_addr,
            database_url,
            google_client_id,
            public_base_url,
            session_ttl_secs,
            run_migrations,
            migrations_path,
        })
    }
}

fn normalize_base_url(value: String) -> Result<String, env::VarError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(env::VarError::NotPresent);
    }
    Ok(trimmed.trim_end_matches('/').to_string())
}
