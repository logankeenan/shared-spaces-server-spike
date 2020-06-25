use handlebars::Handlebars;
use std::collections::BTreeMap;
use actix_web::{HttpResponse, get, web};

#[get("/")]
pub async fn get_root(
    handlebars: web::Data<Handlebars<'_>>
) -> HttpResponse {
    let map: BTreeMap<&str, &str> = BTreeMap::new();

    let body = handlebars.render("root/index", &map).unwrap();

    HttpResponse::Ok().body(body)
}