use tokio::net::TcpListener;

use crate::config::Config;
use crate::http::parser::{read_request, write_response};
use crate::http::response::Response;
use crate::http::router::route;
use crate::state::AppState;
use crate::storage::Storage;

pub async fn run_server(
    config: Config,
    storage: Storage,
) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(&config.bind_addr).await?;
    let state = AppState { config, storage };

    loop {
        let (mut socket, _) = listener.accept().await?;
        let state = state.clone();
        tokio::spawn(async move {
            loop {
                let req = match read_request(&mut socket).await {
                    Ok(Some(r)) => r,
                    Ok(None) => break,
                    Err(code) => {
                        let resp = error_response(code.as_str());
                        let _ = write_response(&mut socket, &resp.to_bytes()).await;
                        break;
                    }
                };

                let response = route(req, state.clone()).await;
                if write_response(&mut socket, &response.to_bytes()).await.is_err() {
                    break;
                }
            }
        });
    }
}

fn error_response(code: &str) -> Response {
    let (status, message) = match code {
        "header_too_large" | "header_count" | "header_parse" | "header_utf8" => {
            (400, "Invalid headers")
        }
        "body_too_large" => (413, "Payload too large"),
        "body_incomplete" | "request_line" | "request_method" | "request_path" => {
            (400, "Invalid request")
        }
        _ => (400, "Bad request"),
    };
    let body = serde_json::json!({"code": "invalid_request", "message": message});
    let json = serde_json::to_vec(&body).unwrap_or_else(|_| b"{}".to_vec());
    Response::json(status, json)
}
