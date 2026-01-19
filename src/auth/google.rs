use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use tokio::sync::RwLock;

static JWK_CACHE: Lazy<RwLock<HashMap<String, Jwk>>> = Lazy::new(|| RwLock::new(HashMap::new()));

#[derive(Deserialize)]
struct JwkSet {
    keys: Vec<Jwk>,
}

#[derive(Deserialize, Clone)]
struct Jwk {
    kid: String,
    kty: String,
    n: String,
    e: String,
}

#[derive(Deserialize)]
pub struct GoogleClaims {
    pub sub: String,
    pub email: String,
    pub email_verified: bool,
    pub aud: String,
    pub iss: String,
    pub exp: usize,
}

pub async fn validate_id_token(
    id_token: &str,
    client_id: &str,
) -> Result<GoogleClaims, String> {
    let header = decode_header(id_token).map_err(|_| "token_invalid")?;
    let kid = header.kid.ok_or("token_invalid")?;
    let jwk = get_key(&kid).await?;
    let decoding_key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e).map_err(|_| "token_invalid")?;

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[client_id]);
    validation.set_issuer(&["accounts.google.com", "https://accounts.google.com"]);

    let token = decode::<GoogleClaims>(id_token, &decoding_key, &validation).map_err(|_| "token_invalid")?;
    let claims = token.claims;
    if !claims.email_verified {
        return Err("token_invalid".to_string());
    }
    Ok(claims)
}

async fn get_key(kid: &str) -> Result<Jwk, String> {
    if let Some(cached) = JWK_CACHE.read().await.get(kid).cloned() {
        return Ok(cached);
    }
    refresh_keys().await?;
    JWK_CACHE
        .read()
        .await
        .get(kid)
        .cloned()
        .ok_or_else(|| "token_invalid".to_string())
}

async fn refresh_keys() -> Result<(), String> {
    let client = Client::new();
    let res = client
        .get("https://www.googleapis.com/oauth2/v3/certs")
        .send()
        .await
        .map_err(|_| "token_invalid")?;
    let set: JwkSet = res.json().await.map_err(|_| "token_invalid")?;
    let mut map = HashMap::new();
    for jwk in set.keys.into_iter().filter(|k| k.kty == "RSA") {
        map.insert(jwk.kid.clone(), jwk);
    }
    let mut cache = JWK_CACHE.write().await;
    *cache = map;
    Ok(())
}
