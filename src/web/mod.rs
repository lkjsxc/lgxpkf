use crate::config::Config;
use crate::http::response::Response;

const HOME_TEMPLATE: &str = include_str!("home.html");

pub fn home_html(config: &Config) -> String {
    render_home(&config.google_client_id)
}

pub fn home(config: &Config) -> Response {
    Response::html(home_html(config))
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
