use actix_web::{Responder, HttpResponse, get, post, http, web};
use handlebars::Handlebars;
use std::collections::{HashMap};
use actix_web::web::Form;
use crate::services::user_service::{get_user};
use actix_identity::Identity;
use crate::services::database::Pool;
use validator::{Validate};
use crate::factories::validation_factory::validation_errors_from;

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct SessionModel {
    #[validate(email(message = "Email is not valid"))]
    #[validate(length(min = 1, message = "Email is required"))]
    pub email: String,

    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
struct SessionViewModel {
    pub form: SessionModel,

    pub errors: HashMap<String, String>,
}

#[get("/session/create")]
pub async fn get_session_create(
    handlebars: web::Data<Handlebars<'_>>
) -> impl Responder {

    let model = SessionViewModel {
        form: SessionModel { email: "".to_string(), password: "".to_string() },
        errors: Default::default(),
    };

    let body = handlebars.render("session/create", &json!(model)).unwrap();

    HttpResponse::Ok().body(body)
}

#[post("/session/save")]
pub async fn post_session_create(
    form: Form<SessionModel>,
    identity: Identity,
    database_pool: web::Data<Pool>,
    handlebars: web::Data<Handlebars<'_>>,
) -> HttpResponse {
    let mut errors = validation_errors_from(form.validate());

    if errors.is_empty() {
        let user = get_user(form.email.to_string(), form.password.to_string(), database_pool);

        match user {
            None => {
                errors.insert("form".to_string(), "Invalid password or email".to_string());

                let model = SessionViewModel {
                    form: SessionModel { email: "".to_string(), password: "".to_string() },
                    errors,
                };

                let body = handlebars.render("session/create", &json!(model)).unwrap();

                HttpResponse::Ok().body(body)
            }
            Some(user) => {
                match user.confirmed_at {
                    None => {
                        return HttpResponse::Found()
                            .header(http::header::LOCATION, "/registration/confirmation-please")
                            .finish();
                    }
                    Some(_) => {
                        identity.remember(user.id.to_owned().to_string());

                        return HttpResponse::Found()
                            .header(http::header::LOCATION, "/app")
                            .finish();
                    }
                }
            }
        }
    } else {
        let model = SessionViewModel {
            form: SessionModel { email: "".to_string(), password: "".to_string() },
            errors,
        };

        let body = handlebars.render("session/create", &json!(model)).unwrap();

        HttpResponse::Ok().body(body)
    }
}

#[post("/session/delete")]
pub async fn delete_session(id: Identity) -> HttpResponse {
    id.forget();

    HttpResponse::Found()
        .header(http::header::LOCATION, "/session/create")
        .finish()
}

