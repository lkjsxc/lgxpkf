use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rand::RngCore;
use time::OffsetDateTime;

pub fn generate_token() -> String {
    let mut bytes = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

pub fn expires_at(ttl_secs: i64) -> OffsetDateTime {
    OffsetDateTime::now_utc() + time::Duration::seconds(ttl_secs)
}
