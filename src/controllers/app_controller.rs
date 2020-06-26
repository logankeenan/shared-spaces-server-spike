use handlebars::Handlebars;
use std::collections::BTreeMap;
use actix_web::{HttpResponse, get, web};
use crate::models::user::User;

#[get("/app")]
pub async fn get_app(
    user: User,
    handlebars: web::Data<Handlebars<'_>>
) -> HttpResponse {
    let map: BTreeMap<&str, &str> = BTreeMap::new();

    let body = handlebars.render("app/index", &map).unwrap();

    HttpResponse::Ok().body(body)
}