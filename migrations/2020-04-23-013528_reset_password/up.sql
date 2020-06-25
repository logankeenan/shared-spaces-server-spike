-- Your SQL goes here
alter table users add column password_reset_at timestamp;
alter table users add column password_reset_token uuid unique;


