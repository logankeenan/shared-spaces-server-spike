use actix_web::{HttpResponse, get, post, web, http};
use handlebars::Handlebars;
use actix_web::web::Form;
use crate::services::user_service::{user_by_password_reset_token, update_user_password, reset_user_password};
use crate::services::email_service::send_password_reset;
use uuid::Uuid;
use std::str::FromStr;
use std::collections::{BTreeMap, HashMap};
use crate::services::database::Pool;
use validator::{Validate};
use crate::factories::validation_factory::validation_errors_from;

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct PasswordResetModel {
    #[validate(email(message = "Email is not valid"))]
    #[validate(length(min = 1, message = "Email is required"))]
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PasswordResetViewModel {
    pub form: PasswordResetModel,
    pub success_message: String,
    pub errors: HashMap<String, String>,
}

#[get("/password/reset")]
pub async fn get_password_rest(
    handlebars: web::Data<Handlebars<'_>>
) -> HttpResponse {
    let model = PasswordResetViewModel {
        form: PasswordResetModel { email: "".to_string() },
        success_message: "".to_string(),
        errors: Default::default(),
    };

    let body = handlebars.render("password/reset", &json!(model)).unwrap();

    HttpResponse::Ok().body(body)
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct ChangePasswordModel {
    #[validate(length(max = 64, message = "Password must be less than 64 characters"))]
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,

    #[validate(must_match(other = "password", message = "Confirm password does not match password"))]
    pub confirm_password: String,
    pub password_reset_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChangePasswordViewModel {
    pub form: ChangePasswordModel,

    pub errors: HashMap<String, String>,
}

#[get("/password/change/{password_reset_token}")]
pub async fn get_password_change(
    password_reset_token: web::Path<String>,
    database_pool: web::Data<Pool>,
    handlebars: web::Data<Handlebars<'_>>,
) -> HttpResponse {
    let password_reset_token_uuid = Uuid::from_str(&password_reset_token).unwrap_or(Uuid::nil());
    let user_option = user_by_password_reset_token(password_reset_token_uuid, database_pool);

    match user_option {
        None => {
            let model: BTreeMap<&str, &str> = BTreeMap::new();
            let body = handlebars.render("password/reset-expired", &model).unwrap();

            HttpResponse::Ok().body(body)
        }
        Some(_) => {
            let model = ChangePasswordViewModel {
                form: ChangePasswordModel {
                    password: "".to_string(),
                    confirm_password: "".to_string(),
                    password_reset_token: password_reset_token.to_string(),
                },
                errors: HashMap::new(),
            };

            let body = handlebars.render("password/change", &json!(model)).unwrap();

            HttpResponse::Ok().body(body)
        }
    }
}

#[post("/password/change-save")]
pub async fn post_password_change(
    form: Form<ChangePasswordModel>,
    database_pool: web::Data<Pool>,
    handlebars: web::Data<Handlebars<'_>>,
) -> HttpResponse {
    let errors = validation_errors_from(form.validate());

    if errors.is_empty() {
        let result = Uuid::from_str(form.password_reset_token.as_str()).unwrap_or(Uuid::nil());
        // removing this causes and error "use of moved value" for the next usages of database_pool.
        let user_option = user_by_password_reset_token(result, database_pool.clone());

        match user_option {
            None => {
                let model: BTreeMap<&str, &str> = BTreeMap::new();
                let body = handlebars.render("password/reset-expired", &model).unwrap();

                return HttpResponse::Ok().body(body);
            }
            Some(user) => {
                update_user_password(user.id, form.password.to_string(), database_pool);

                return HttpResponse::Found()
                    .header(http::header::LOCATION, "/session/create")
                    .finish();
            }
        }
    }

    let model = ChangePasswordViewModel {
        form: ChangePasswordModel {
            password: "".to_string(),
            confirm_password: "".to_string(),
            password_reset_token: form.password_reset_token.to_string(),
        },
        errors,
    };

    let body = handlebars.render("password/change", &json!(model)).unwrap();

    HttpResponse::Ok().body(body)
}

#[post("password/reset-save")]
pub async fn post_password_reset(
    form: Form<PasswordResetModel>,
    database_pool: web::Data<Pool>,
    handlebars: web::Data<Handlebars<'_>>,
) -> HttpResponse {
    let errors = validation_errors_from(form.validate());

    if errors.is_empty() {
        let model = PasswordResetViewModel {
            form: PasswordResetModel { email: "".to_string() },
            success_message: "An email has been sent to reset your password".to_string(),
            errors: Default::default(),
        };

        let user_option = reset_user_password(form.email.to_string(), database_pool);

        match user_option {
            None => {}
            Some(_user) => {
                send_password_reset(_user).await;
            }
        }

        let body = handlebars.render("password/reset", &json!(model)).unwrap();

        HttpResponse::Ok().body(body)
    } else {
        let model = PasswordResetViewModel {
            form: PasswordResetModel { email: form.email.to_string() },
            success_message: "".to_string(),
            errors,
        };

        let body = handlebars.render("password/reset", &json!(model)).unwrap();

        HttpResponse::Ok().body(body)
    }
}
