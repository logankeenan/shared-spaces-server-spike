table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        hash -> Bytea,
        salt -> Varchar,
        password_reset_at -> Nullable<Timestamp>,
        password_reset_token -> Nullable<Uuid>,
        confirmed_at -> Nullable<Timestamp>,
        confirmation_sent_at -> Timestamp,
        confirmation_token -> Uuid,
    }
}
