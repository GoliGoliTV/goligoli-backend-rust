use actix_web::{HttpRequest};

pub fn user_test(_req: &HttpRequest) -> &'static str {
    "User Hello world!"
}