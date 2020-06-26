#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate handlebars;

#[macro_use]
extern crate validator_derive;
extern crate validator;

extern crate env_logger;

use actix_web::{App, HttpServer, web, middleware};
use actix_files as fs;
use std::env;
use listenfd::ListenFd;
use crate::controllers::registration_controller::{get_registration_create, post_registration_save, get_registration_confirmation_sent, get_registration_confirmation_create, post_registration_confirmation_save, get_registration_confirmation, get_registration_confirmation_please};
use crate::controllers::user_controller::{get_user_details, get_user_edit, post_user_update};
use actix_identity::{CookieIdentityPolicy, IdentityService};
use crate::controllers::session_controller::{get_session_create, post_session_create, delete_session};
use dotenv;
use crate::controllers::password_controller::{get_password_rest, post_password_reset, get_password_change, post_password_change};
use crate::controllers::root_controller::get_root;
use std::str::FromStr;
use actix_web::cookie::SameSite;
use crate::services::database::{create_database_pool};
use handlebars::Handlebars;
use actix_web::middleware::Logger;
use crate::pages::handlebar_helpers::register_helpers;
use std::borrow::BorrowMut;
use crate::controllers::app_controller::get_app;

mod schema;
mod models;
mod services;
mod controllers;
mod factories;
mod traits;
mod pages;



#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    load_environment_variables();
    let mut listenfd = ListenFd::from_env();

    let port: String = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    let domain = env::var("HOST")
        .unwrap_or_else(|_| "localhost".to_string());

    //TODO all these ways to get env vars needs to be in a factory
    let is_secure: bool = FromStr::from_str(env::var("IS_SECURE")
        .unwrap_or_else(|_| "false".to_string()).as_ref()).unwrap();

    let master_key = env::var("MASTER_KEY").unwrap().clone();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let database_pool = create_database_pool(database_url.to_string());
    let web_database_pool = web::Data::new(database_pool);

    let mut handlebars = Handlebars::new();

    register_helpers(handlebars.borrow_mut());

    handlebars
        .register_templates_directory(".hbs", "./src/pages")
        .unwrap();

    let handlebars_ref = web::Data::new(handlebars);

    env_logger::init();

    let mut server = HttpServer::new(move ||
        {
            let app = App::new();
            app
                .wrap(middleware::Compress::default())
                .app_data(handlebars_ref.clone())
                .app_data(web_database_pool.clone())
                .service(get_root)
                .service(get_app)
                .service(get_registration_create)
                .service(post_registration_save)
                .service(get_user_details)
                .service(get_session_create)
                .service(post_session_create)
                .service(get_password_rest)
                .service(post_password_reset)
                .service(get_password_change)
                .service(post_password_change)
                .service(get_registration_confirmation_sent)
                .service(get_registration_confirmation_create)
                .service(post_registration_confirmation_save)
                .service(get_registration_confirmation)
                .service(get_registration_confirmation_please)
                .service(delete_session)
                .service(get_user_edit)
                .service(fs::Files::new("/static/shared_space_app.js", "node_modules/@logankeenan/shared-space-app/shared_space_app.js"))
                .service(fs::Files::new("/static/shared_space_app_bg.wasm", "node_modules/@logankeenan/shared-space-app/shared_space_app_bg.wasm"))
                .service(fs::Files::new("/static", "static"))
                .service(fs::Files::new("/node_modules", "node_modules"))
                .service(post_user_update)
                .wrap(Logger::default())
                .wrap(IdentityService::new(
                    CookieIdentityPolicy::new(master_key.to_string().as_bytes())    // <- create cookie identity policy
                        .name("auth-cookie")
                        .domain(domain.clone())
                        .same_site(SameSite::Strict)
                        .max_age_time(chrono::Duration::days(60))
                        .secure(is_secure))
                )

        }
    );

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        let address = format!("0.0.0.0:{}", port);

        server.bind(address)?
    };

    server.run().await
}

fn load_environment_variables() {
    let environment = env::var("RUST_ENV").unwrap_or_else(|_| "".to_string());

    if environment == "development" {
        dotenv::from_filename(".env.development").ok();
    }
    if environment == "test" {
        dotenv::from_filename(".env.test").ok();
    }
    if environment == "production" {
        dotenv::from_filename(".env.production").ok();
    }
    dotenv::dotenv().ok();
}