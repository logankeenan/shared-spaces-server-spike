use actix_web::{get, post, http, Responder, HttpResponse, web};
use handlebars::Handlebars;
use std::collections::{BTreeMap, HashMap};
use actix_web::web::Form;
use crate::services::user_service::{create_user, confirm_user_by_confirmation_token, reissue_confirmation_token};
use crate::services::email_service::send_email_confirmation;
use uuid::Uuid;
use std::str::FromStr;
use crate::services::database::Pool;
use validator::{Validate};
use crate::factories::validation_factory::validation_errors_from;

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct RegistrationModel {
    #[validate(length(max = 50, message = "First name must be less than 50 characters"))]
    #[validate(length(min = 1, message = "First name is required"))]
    pub first_name: String,

    //TODO this should probably have a larger max
    #[validate(length(max = 50, message = "Last name must be less than 50 characters"))]
    #[validate(length(min = 1, message = "Last name is required"))]
    pub last_name: String,

    //TODO this should probably have a max
    #[validate(email(message = "Email is not valid"))]
    #[validate(length(min = 1, message = "Email is required"))]
    pub email: String,

    #[validate(length(max = 64, message = "Password must be less than 64 characters"))]
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,

    #[validate(must_match(other = "password", message = "Confirm password does not match password"))]
    pub confirm_password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegistrationCreateViewModel {
    pub form: RegistrationModel,

    pub errors: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct ConfirmationCreateModel {
    #[validate(email(message = "Email is not valid"))]
    #[validate(length(min = 1, message = "Email is required"))]
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfirmationCreateViewModel {
    pub form: ConfirmationCreateModel,
    pub success_message: String,
    pub errors: HashMap<String, String>,
}


#[get("/registration/create")]
pub async fn get_registration_create(
    handlebars: web::Data<Handlebars<'_>>
) -> impl Responder {
    let map: BTreeMap<&str, &str> = BTreeMap::new();

    let body = handlebars.render("registration/create", &map).unwrap();

    HttpResponse::Ok().body(body)
}

#[get("/registration/confirmation-sent")]
pub async fn get_registration_confirmation_sent(
    handlebars: web::Data<Handlebars<'_>>
) -> impl Responder {
    let model: BTreeMap<&str, &str> = BTreeMap::new();

    let body = handlebars.render("registration/confirmation-sent", &model).unwrap();

    HttpResponse::Ok().body(body)
}

#[get("/registration/confirmation-create")]
pub async fn get_registration_confirmation_create(
    handlebars: web::Data<Handlebars<'_>>
) -> impl Responder {
    let model: BTreeMap<&str, &str> = BTreeMap::new();

    let body = handlebars.render("registration/confirmation-create", &model).unwrap();

    HttpResponse::Ok().body(body)
}

#[post("/registration/confirmation-save")]
pub async fn post_registration_confirmation_save(
    form: Form<ConfirmationCreateModel>,
    database_pool: web::Data<Pool>,
    handlebars: web::Data<Handlebars<'_>>,
) -> HttpResponse {
    let errors = validation_errors_from(form.validate());
    if errors.is_empty() {
        let model = ConfirmationCreateViewModel {
            form: ConfirmationCreateModel { email: "".to_string() },
            success_message: "An email has been sent to confirm your email address".to_string(),
            errors: Default::default(),
        };

        let user_option = reissue_confirmation_token(form.email.to_string(), database_pool);
        match user_option {
            None => {}
            Some(user) => {
                send_email_confirmation(user).await
            }
        }

        let body = handlebars.render("registration/confirmation-create", &model).unwrap();

        HttpResponse::Ok().body(body)
    } else {
        let model = ConfirmationCreateViewModel {
            form: ConfirmationCreateModel {
                email: form.email.to_string()
            },
            success_message: "".to_string(),
            errors,
        };

        let body = handlebars.render("registration/confirmation-create", &model).unwrap();

        HttpResponse::Ok().body(body)
    }
}

#[get("/registration/confirmation-please")]
pub async fn get_registration_confirmation_please(
    handlebars: web::Data<Handlebars<'_>>
) -> impl Responder {
    let model: BTreeMap<&str, &str> = BTreeMap::new();
    let body = handlebars.render("registration/confirmation-please", &model).unwrap();

    HttpResponse::Ok().body(body)
}

#[get("/registration/confirmation/{confirmation_token}")]
pub async fn get_registration_confirmation(
    confirmation_token: web::Path<String>,
    database_pool: web::Data<Pool>,
    handlebars: web::Data<Handlebars<'_>>
) -> HttpResponse {
    let confirmation_token_uuid = Uuid::from_str(&confirmation_token).unwrap_or(Uuid::nil());
    let user_option = confirm_user_by_confirmation_token(confirmation_token_uuid, database_pool);

    match user_option {
        None => {
            let model: BTreeMap<&str, &str> = BTreeMap::new();
            let body = handlebars.render("registration/confirmation-expired", &model).unwrap();

            HttpResponse::Ok().body(body)
        }
        Some(_) => {
            let model: BTreeMap<&str, &str> = BTreeMap::new();

            let body = handlebars.render("registration/confirmation-success", &json!(model)).unwrap();

            HttpResponse::Ok().body(body)
        }
    }
}

#[post("/registration/save")]
pub async fn post_registration_save(
    form: Form<RegistrationModel>,
    database_pool: web::Data<Pool>,
    handlebars: web::Data<Handlebars<'_>>
) -> HttpResponse {
    let errors = validation_errors_from(form.validate());
    if errors.is_empty() {
        let new_user = create_user(&form, database_pool);

        match new_user {
            None => {}
            Some(user) => {
                send_email_confirmation(user).await;
            }
        }

        return HttpResponse::Found()
            .header(http::header::LOCATION, "/registration/confirmation-sent")
            .finish();
    }

    let mut _model = RegistrationCreateViewModel {
        form: RegistrationModel {
            first_name: form.first_name.to_string(),
            last_name: form.last_name.to_string(),
            email: form.email.to_string(),
            password: form.password.to_string(),
            confirm_password: form.confirm_password.to_string(),
        },
        errors,
    };

    let body = handlebars.render("registration/create", &json!(_model)).unwrap();

    HttpResponse::Ok().body(body)
}