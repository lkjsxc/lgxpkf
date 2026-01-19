use data_encoding::BASE32_NOPAD;

pub fn encode_id(id: [u8; 32]) -> String {
    BASE32_NOPAD.encode(&id).to_lowercase()
}

pub fn decode_id(encoded: &str) -> Option<[u8; 32]> {
    if encoded.len() != 52 {
        return None;
    }
    let upper = encoded.to_uppercase();
    let bytes = BASE32_NOPAD.decode(upper.as_bytes()).ok()?;
    if bytes.len() != 32 {
        return None;
    }
    let mut out = [0u8; 32];
    out.copy_from_slice(&bytes);
    Some(out)
}

pub fn is_base32_url(value: &str) -> bool {
    if value.len() != 52 {
        return false;
    }
    value
        .chars()
        .all(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '2'..='7'))
}
