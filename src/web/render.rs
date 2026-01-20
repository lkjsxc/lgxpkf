use crate::config::Config;
use crate::web::escape::escape_attr;
use crate::web::templates;

pub fn home_html(config: &Config) -> String {
    render_home(&config.google_client_id, &login_uri(config), "home")
}

pub fn signin_html(config: &Config) -> String {
    render_signin(&config.google_client_id, &login_uri(config))
}

pub fn redirect_html(token: &str, target: &str) -> String {
    let token = escape_attr(token);
    let target = escape_attr(target);
    templates::REDIRECT
        .replace("{{TOKEN}}", &token)
        .replace("{{TARGET}}", &target)
}

pub fn login_uri(config: &Config) -> String {
    format!("{}/auth/google/redirect", config.public_base_url)
}

fn render_home(client_id: &str, login_uri: &str, view: &str) -> String {
    let client_id = escape_attr(client_id);
    let login_uri = escape_attr(login_uri);
    let view = escape_attr(view);
    templates::HOME
        .replace("{{CLIENT_ID}}", &client_id)
        .replace("{{LOGIN_URI}}", &login_uri)
        .replace("{{VIEW}}", &view)
}

fn render_signin(client_id: &str, login_uri: &str) -> String {
    let client_id = escape_attr(client_id);
    let login_uri = escape_attr(login_uri);
    templates::SIGNIN
        .replace("{{CLIENT_ID}}", &client_id)
        .replace("{{LOGIN_URI}}", &login_uri)
}
