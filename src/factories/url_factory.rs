use url::Url;
use std::str::FromStr;
use std::env;

pub fn create_url(path: String) -> Url {
    let is_secure: bool = FromStr::from_str(env::var("IS_SECURE")
        .unwrap_or_else(|_| "false".to_string()).as_ref()).unwrap();

    let mut port: String = env::var("PORT")
        .unwrap_or_else(|_| "".to_string())
        .parse()
        .expect("PORT must be a number");

    let mut schema = "http://";
    if is_secure {
        schema = "https://"
    }

    if !port.eq("") {
        port = format!(":{}", port);
    }

    // TOOD this is shit and the env var factory should have a method called port_for_url
    // that hides this logic.  The prod port is some random assigned port by heroku
    // thought env var which messes up the url creation.
    let environment = env::var("RUST_ENV").unwrap_or_else(|_| "".to_string());
    if environment == "production" {
        port = "".to_string();
    }

    let host = env::var("HOST")
        .unwrap_or_else(|_| "localhost".to_string());

    let url_as_string = format!("{}{}{}", schema, host, port);

    let mut url = Url::parse(url_as_string.as_str()).unwrap();

    url.set_path(path.as_str());

    url
}