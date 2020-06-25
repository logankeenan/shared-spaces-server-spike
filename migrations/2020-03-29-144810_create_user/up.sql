-- Your SQL goes here

create table users
(
    id         SERIAL PRIMARY KEY,
    first_name varchar      not null,
    last_name  varchar      not null,
    email      varchar(120) not null unique,
    created_at timestamp    not null,
    updated_at timestamp,
    hash BYTEA NOT NULL,
    salt VARCHAR(255) NOT NULL
);

CREATE UNIQUE INDEX users__email_idx ON users(email);