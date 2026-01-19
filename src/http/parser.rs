use bytes::BytesMut;
use std::collections::HashMap;
use std::str;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

const MAX_HEADER_BYTES: usize = 32 * 1024;
const MAX_HEADER_COUNT: usize = 100;
const MAX_BODY_BYTES: usize = 1024 * 1024;

#[derive(Clone)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub query: Option<String>,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

pub async fn read_request(stream: &mut TcpStream) -> Result<Option<Request>, String> {
    let mut buffer = BytesMut::with_capacity(4096);
    loop {
        if buffer.len() > MAX_HEADER_BYTES {
            return Err("header_too_large".to_string());
        }
        let n = stream.read_buf(&mut buffer).await.map_err(|_| "read_error")?;
        if n == 0 {
            return Ok(None);
        }
        if buffer.windows(4).any(|w| w == b"\r\n\r\n") {
            break;
        }
    }

    let header_end = buffer
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .ok_or("header_parse")?;
    let header_bytes = &buffer[..header_end];
    let header_str = str::from_utf8(header_bytes).map_err(|_| "header_utf8")?;
    let mut lines = header_str.split("\r\n");
    let request_line = lines.next().ok_or("request_line")?;
    let mut parts = request_line.split_whitespace();
    let method = parts.next().ok_or("request_method")?.to_string();
    let full_path = parts.next().ok_or("request_path")?.to_string();
    let _version = parts.next().ok_or("request_version")?;

    let mut headers = HashMap::new();
    for (idx, line) in lines.enumerate() {
        if idx >= MAX_HEADER_COUNT {
            return Err("header_count".to_string());
        }
        if let Some((name, value)) = line.split_once(':') {
            headers.insert(
                name.trim().to_ascii_lowercase(),
                value.trim().to_string(),
            );
        } else {
            return Err("header_parse".to_string());
        }
    }

    let (path, query) = if let Some((p, q)) = full_path.split_once('?') {
        (p.to_string(), Some(q.to_string()))
    } else {
        (full_path, None)
    };

    let content_length = headers
        .get("content-length")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);

    if content_length > MAX_BODY_BYTES {
        return Err("body_too_large".to_string());
    }

    let mut body = Vec::with_capacity(content_length);
    let already = buffer[(header_end + 4)..].to_vec();
    body.extend_from_slice(&already);

    while body.len() < content_length {
        let mut chunk = vec![0u8; content_length - body.len()];
        let n = stream.read(&mut chunk).await.map_err(|_| "read_error")?;
        if n == 0 {
            break;
        }
        body.extend_from_slice(&chunk[..n]);
    }

    if body.len() != content_length {
        return Err("body_incomplete".to_string());
    }

    Ok(Some(Request {
        method,
        path,
        query,
        headers,
        body,
    }))
}

pub async fn write_response(stream: &mut TcpStream, bytes: &[u8]) -> Result<(), String> {
    stream
        .write_all(bytes)
        .await
        .map_err(|_| "write_error".to_string())
}
