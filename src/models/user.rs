use crate::schema::*;
use diesel::{Queryable, Insertable, Identifiable};
use chrono::*;
use uuid::Uuid;


#[derive(Identifiable, Debug, Serialize, Deserialize, Queryable, Clone)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub hash: Vec<u8>,
    pub salt: String,
    pub password_reset_at: Option<NaiveDateTime>,
    pub password_reset_token: Option<Uuid>,
    pub confirmed_at: Option<NaiveDateTime>,
    pub confirmation_sent_at: NaiveDateTime,
    pub confirmation_token: Uuid,
}


#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct InsertableUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub hash: Vec<u8>,
    pub salt: String,
    pub confirmed_at: Option<NaiveDateTime>,
    pub confirmation_sent_at: NaiveDateTime,
    pub confirmation_token: Uuid,
}