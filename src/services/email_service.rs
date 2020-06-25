use crate::models::user::User;
use mailgun_v3::email::{MessageBody};
use mailgun_v3::{Credentials, EmailAddress};
use mailgun_v3::email::Message;
use std::env;
use mailgun_v3::email::async_impl::send_email;
use crate::factories::url_factory::create_url;

pub async fn send_password_reset(user: User) {
    let to_address = EmailAddress::address(user.email.as_str());
    let reset_token = user.password_reset_token.unwrap().to_hyphenated().to_string();

    let url = create_url(format!("/password/change/{}", reset_token));

    let message_body = MessageBody::Html(format!(
        "Hi {}, \
        <br><br> \
        Please click <a href=\"{}\">here</a> to reset your password.\
        <br><br>\
        Thanks, <br>\
        Shared Spaces", user.first_name, url.to_string()).to_string());

    let message = Message {
        to: vec![to_address],
        cc: vec![],
        bcc: vec![],
        subject: "Password Reset".to_string(),
        body: message_body,
        options: vec![],
    };
    let result = send_email(&get_credentials(), &get_sender_address(), message).await;

    let environment = env::var("RUST_ENV").unwrap_or_else(|_| "".to_string());
    if environment == "test" {
        return;
    }

    match result {
        Ok(_) => {
            //TODO What should I do with the SendResponse. Log it? https://docs.rs/mailgun_v3/0.9.0/mailgun_v3/email/struct.SendResponse.html
        }
        Err(error) => {
            //TODO Need to figure out how log out the body of the error.
            println!("Send Mail Error: {}", error)
        }
    }
}

fn get_credentials() -> Credentials {
    let mailgun_api_key = env::var("MAILGUN_API_KEY").unwrap();

    let credentials = Credentials::new(
        mailgun_api_key.as_str(),
        "cultivatedsoftware.com",
    );
    credentials
}

fn get_sender_address() -> EmailAddress {
    EmailAddress::name_address("Shared Spaces", "do-not-reply@mail.sharedspaces.app")
}

pub async fn send_email_confirmation(user: User) {
    let to_address = EmailAddress::address(user.email.as_str());


    let confirmation_token = user.confirmation_token.to_hyphenated().to_string();
    let url = create_url(format!("/registration/confirmation/{}", confirmation_token));


    let message_body = MessageBody::Html(format!(
        "Hi {}, \
        <br><br> \
        Please click <a href=\"{}\">here</a> to confirm your email address.\
        <br><br>\
        Thanks, <br>\
        Shared Spaces", user.first_name, url.to_string()).to_string());

    let message = Message {
        to: vec![to_address],
        cc: vec![],
        bcc: vec![],
        subject: "Confirm Email Address".to_string(),
        body: message_body,
        options: vec![],
    };

    let environment = env::var("RUST_ENV").unwrap_or_else(|_| "".to_string());

    if environment == "test" {
        return;
    }

    let result = send_email(&get_credentials(), &get_sender_address(), message).await;

    match result {
        Ok(_) => {
            //TODO What should I do with the SendResponse. Log it? https://docs.rs/mailgun_v3/0.9.0/mailgun_v3/email/struct.SendResponse.html
        }
        Err(error) => {
            //TODO Need to figure out how log out the body of the error.
            println!("Send Mail Error: {}", error)
        }
    }
}