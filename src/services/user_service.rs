use crate::models::user::{User, InsertableUser};
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods, BoolExpressionMethods, QueryResult};
use argon2rs::argon2i_simple;
use crate::controllers::registration_controller::RegistrationModel;
use std::borrow::Borrow;
use crate::schema::users::table as user_table;
use crate::schema::users::columns as user_columns;
use uuid::Uuid;
use std::ops::Sub;
use crate::services::database::{Pool, establish_connection_from, PooledConnection};
use actix_web::web;

pub fn make_salt() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 32;
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    password
}

pub fn make_hash(password: &str, salt: &str) -> [u8; argon2rs::defaults::LENGTH] {
    argon2i_simple(password, salt)
}


pub fn create_user(user_form: &RegistrationModel, pool: web::Data<Pool>) -> Option<User> {
    let salt = make_salt();
    let hash = make_hash(&user_form.password, &salt).to_vec();

    let insertable_user = InsertableUser {
        first_name: user_form.first_name.to_string(),
        last_name: user_form.last_name.to_string(),
        email: user_form.email.to_string(),
        hash,
        salt,
        confirmed_at: None,
        created_at: chrono::Utc::now().naive_local(),
        updated_at: None,
        confirmation_token: Uuid::new_v4(),
        confirmation_sent_at: chrono::Utc::now().naive_local(),
    };

    let user_by_email: Option<User> = user_table
        .filter(user_columns::email.eq(user_form.email.to_string()))
        .first(&establish_connection_from(&pool)?).ok();

    match user_by_email {
        None => {

            //TODO this panics when running the integration tests only if the database has just
            // been reset. It doesn't do it on subsequent runs.
            let user: User = diesel::insert_into(user_table)
                .values(&insertable_user)
                .get_result(&establish_connection_from(&pool)?)
                .expect("Error saving new post");

            Some(user)
        }
        Some(_) => {
            None
        }
    }
}

fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}

pub fn user_by_id(id: i32, pool: PooledConnection) -> Option<User> {
    let user: Option<User> = user_table.find(id).first(&pool).ok();

    user
}

pub fn reissue_confirmation_token(email: String, pool: web::Data<Pool>) -> Option<User> {
    let current_time = chrono::Utc::now().naive_local();

    let update_result = diesel::update(user_table.filter(user_columns::email.eq(email)))
        .set((
            user_columns::confirmation_sent_at.eq(current_time),
            user_columns::updated_at.eq(current_time),
            user_columns::confirmation_token.eq(Uuid::new_v4())
        )).get_result::<User>(&establish_connection_from(&pool)?);

    match update_result {
        Ok(user) => {
            Some(user)
        }
        Err(_) => {
            None
        }
    }
}

pub fn get_user(email_param: String, password: String, pool: web::Data<Pool>) -> Option<User> {
    use crate::schema::users::dsl::*;

    let user_query: QueryResult<User> = users.filter(email.eq(email_param))
        .get_result::<User>(&establish_connection_from(&pool)?);

    match user_query {
        Ok(user) => {
            let hash_from_provided_password = make_hash(password.as_str(), user.salt.as_str());
            let password_provided_hash_vec: Vec<u8> = hash_from_provided_password.to_vec();

            if do_vecs_match(user.hash.borrow(), &password_provided_hash_vec) {
                return Some(user);
            }
            return None;
        }
        Err(_) => {
            return None;
        }
    }
}

pub fn reset_user_password(email_param: String, pool: web::Data<Pool>) -> Option<User> {
    let user_result = user_table.filter(user_columns::email.eq(email_param));

    let current_time = chrono::Utc::now().naive_local();

    let update_result = diesel::update(user_result).set((
        user_columns::password_reset_token.eq(Uuid::new_v4()),
        user_columns::password_reset_at.eq(current_time),
        user_columns::updated_at.eq(current_time))
    ).get_result::<User>(&establish_connection_from(&pool)?);

    match update_result {
        Ok(user) => {
            Some(user)
        }
        Err(_) => {
            None
        }
    }
}

pub fn user_by_password_reset_token(password_reset_token: Uuid, pool: web::Data<Pool>) -> Option<User> {
    let allow_one_day_for_token = chrono::Utc::now().sub(chrono::Duration::days(1));

    let user_result = user_table.filter(
        user_columns::password_reset_token.eq(password_reset_token)
            .and(user_columns::password_reset_at.gt(allow_one_day_for_token.naive_local()))
    ).get_result::<User>(&establish_connection_from(&pool)?);

    match user_result {
        Ok(user) => {
            Some(user)
        }
        Err(_) => {
            None
        }
    }
}

pub fn confirm_user_by_confirmation_token(confirmation_token: Uuid, pool: web::Data<Pool>) -> Option<User> {
    let allow_one_day_for_token = chrono::Utc::now().sub(chrono::Duration::days(1));
    let current_time = chrono::Utc::now().naive_local();

    let user_result = diesel::update(
        user_table.filter(
            user_columns::confirmation_token.eq(confirmation_token)
                .and(user_columns::confirmation_sent_at.gt(allow_one_day_for_token.naive_local()))))
        .set((
            user_columns::confirmed_at.eq(current_time),
            user_columns::updated_at.eq(current_time),
        ))
        .get_result::<User>(&establish_connection_from(&pool)?);

    match user_result {
        Ok(user) => {
            Some(user)
        }
        Err(_) => {
            None
        }
    }
}

pub fn update_user_password(user_id: i32, password: String, pool: web::Data<Pool>) {
    let salt = make_salt();
    let hash = make_hash(password.as_str(), &salt).to_vec();

    diesel::update(user_table.find(user_id)).set((
        user_columns::hash.eq(hash),
        user_columns::salt.eq(salt),
        user_columns::updated_at.eq(chrono::Utc::now().naive_local()))
    ).execute(&establish_connection_from(&pool).unwrap()).unwrap();
}

pub fn update_user(user: User, pool: web::Data<Pool>) -> Option<User> {
    let result = diesel::update(user_table.find(user.id))
        .set((
            user_columns::first_name.eq(user.first_name),
            user_columns::last_name.eq(user.last_name),
            user_columns::updated_at.eq(chrono::Utc::now().naive_local()),
        ))
        .get_result::<User>(&establish_connection_from(&pool)?).ok();

    result
}
