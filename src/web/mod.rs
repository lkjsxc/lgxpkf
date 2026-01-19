use crate::config::Config;
use crate::http::response::Response;

const HOME_TEMPLATE: &str = include_str!("home.html");

pub fn home(config: &Config) -> Response {
    let body = render_home(&config.google_client_id);
    Response::html(body)
}

fn render_home(client_id: &str) -> String {
    let client_id = escape_attr(client_id);
    HOME_TEMPLATE.replace("{{CLIENT_ID}}", &client_id)
}

fn escape_attr(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\'', "&#39;")
}
