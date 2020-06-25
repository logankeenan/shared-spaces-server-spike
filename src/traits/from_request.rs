use actix_web::{FromRequest, Error, HttpRequest};
use crate::models::user::User;
use actix_web::dev::Payload;
use actix_identity::RequestIdentity;
use actix_web::web::Data;
use crate::services::database::{Pool, establish_connection_from};
use crate::services::user_service::{user_by_id};
use actix_web::error::ErrorUnauthorized;

impl FromRequest for User {
    type Error = Error;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let identity = req.get_identity();
        let option = req.app_data::<Data<Pool>>();


        match option {
            None => {}
            Some(pool) => {

                match identity {
                    None => {},
                    Some(user_id_string) => {
                        let user_id: i32 = user_id_string.parse().unwrap_or(0);
                        let connection_pool = establish_connection_from(&pool).unwrap();
                        let user_options = user_by_id(user_id, connection_pool);
                        match user_options {
                            None => {}
                            Some(user) => {
                                return futures::future::ready(std::result::Result::Ok(user));
                            }
                        }
                    },
                }
            }
        }

        futures::future::ready(std::result::Result::Err(ErrorUnauthorized("Unauthorized")))
    }
}