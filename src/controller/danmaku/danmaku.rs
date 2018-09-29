use actix_web::{error, http, middleware, server, App, AsyncResponder, Error, HttpMessage,HttpRequest, HttpResponse, Json,};
use std::time::SystemTime;

use bytes::BytesMut;
use futures::{Future, Stream};
use json::JsonValue;

pub fn index(_req: &HttpRequest) -> &'static str {
    "Hello world!"
}

pub fn get_danmaku_by_video(_req: &HttpRequest) -> &'static str {
    "test"
}

#[derive(Debug, Serialize, Deserialize)]
struct DanmakuContent {
    time:i64,
    content:String,
    video_id:i32,
}

pub fn send_danmaku(_req: &HttpRequest) -> Box<Future<Item = HttpResponse, Error = Error>> {
    _req.json()
        .from_err()
        .and_then(|val: DanmakuContent|{
            println!("model:{:?}",val);
            Ok(HttpResponse::Ok().json(val))
        })
        .responder()
}