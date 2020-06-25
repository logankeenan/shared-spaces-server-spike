use actix_web::{Responder, HttpResponse, get, post, http, web};
use crate::services::user_service::{update_user};
use crate::services::database::Pool;
use crate::models::user::User;
use handlebars::Handlebars;
use std::collections::HashMap;
use validator::{Validate};
use actix_web::web::Form;
use std::borrow::Borrow;
use crate::factories::validation_factory::validation_errors_from;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDetailsViewModel {
    pub current_user: User,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct UserEditModel {
    #[validate(length(max = 50, message = "First name must be less than 50 characters"))]
    #[validate(length(min = 1, message = "First name is required"))]
    pub first_name: String,

    #[validate(length(max = 50, message = "Last name must be less than 50 characters"))]
    #[validate(length(min = 1, message = "Last name is required"))]
    pub last_name: String,

    pub id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserEditViewModel {
    pub form: UserEditModel,
    pub current_user: User,
    pub errors: HashMap<String, String>,
}

#[get("/users/{id}")]
pub async fn get_user_details(
    user_id: web::Path<i32>,
    current_user: User,
    handlebars: web::Data<Handlebars<'_>>,
) -> impl Responder {

    //TODO an individual can only view themselves, so this works but won't if others
    if current_user.id.eq(user_id.into_inner().borrow()) {
        let model = UserDetailsViewModel {
            current_user
        };

        let body = handlebars.render("user/details", &json!(model)).unwrap();

        HttpResponse::Ok().body(body)
    } else {
        redirect_to_login()
    }
}

fn redirect_to_login() -> HttpResponse {
    HttpResponse::Found()
        .header(http::header::LOCATION, "/session/create")
        .finish()
}

#[get("/users/{id}/edit")]
pub async fn get_user_edit(
    user_id: web::Path<i32>,
    current_user: User,
    handlebars: web::Data<Handlebars<'_>>,
) -> impl Responder {
    if current_user.id.eq(user_id.into_inner().borrow()) {
        let model = UserEditViewModel {
            form: UserEditModel {
                first_name: current_user.first_name.to_string(),
                last_name: current_user.last_name.to_string(),
                id: current_user.id,
            },
            current_user,
            errors: Default::default(),
        };

        let body = handlebars.render("user/edit", &json!(model)).unwrap();

        HttpResponse::Ok().body(body)
    } else {
        redirect_to_login()
    }
}


#[post("/users/update")]
pub async fn post_user_update(
    form: Form<UserEditModel>,
    mut user: User,
    database_pool: web::Data<Pool>,
    handlebars: web::Data<Handlebars<'_>>, ) -> impl Responder {

    if user.id.eq(form.id.borrow()) {
        let errors = validation_errors_from(form.validate());

        if errors.is_empty() {
            user.first_name = form.first_name.to_string();
            user.last_name = form.last_name.to_string();

            let user_id = user.id.clone();
            let user_option = update_user(user, database_pool);

            match user_option {
                None => {
                    // TODO show some error message page
                    return redirect_to_login();
                }
                Some(_) => {
                    return HttpResponse::Found()
                        .header(http::header::LOCATION, format!("/users/{}", user_id))
                        .finish();
                }
            }
        } else {
            let model = UserEditViewModel {
                form: UserEditModel {
                    first_name: form.first_name.to_string(),
                    last_name: form.last_name.to_string(),
                    id: user.id,
                },
                current_user: user,
                errors,
            };

            let body = handlebars.render("user/edit", &json!(model)).unwrap();

            HttpResponse::Ok().body(body)
        }
    } else {
        return redirect_to_login();
    }
}


