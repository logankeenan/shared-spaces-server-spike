-- Your SQL goes here
alter table users add column confirmed_at timestamp;
alter table users add column confirmation_sent_at timestamp not null;
alter table users add column confirmation_token uuid unique not null;
alter table users add constraint email_unique_index UNIQUE (email);