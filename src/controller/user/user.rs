use actix_web::{error, http, middleware, server, App, AsyncResponder, Error, HttpMessage,HttpRequest, HttpResponse, Json,};
use std::time::SystemTime;

use bytes::BytesMut;
use futures::{Future, Stream};
use json::JsonValue;

#[derive(Debug, Serialize, Deserialize)]
struct SignUpReq {
    name: String,
}

pub fn user_signup(_req: &HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    _req.json().from_err()
        .and_then(|val: SignUpReq| {
            println!("model: {:?}", val);
            Ok(HttpResponse::Ok().json(val))  // <- send response
        })
        .responder()
}

pub fn user_signin(_req: &HttpRequest) -> &'static str {
    "ddd"
}

pub fn user_zone(_req: &HttpRequest) -> &'static str {
    "dddddd"
}

fn create_user() {
    
}