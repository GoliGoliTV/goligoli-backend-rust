use actix_web::{server, App, HttpRequest};

mod user;
mod danmaku;
mod video;

fn index(_req: &HttpRequest) -> &'static str {
    "Hello world!"
}

fn main() {
    server::new(|| {
        App::new()
            .resource("/", |r| r.f(index))
            .configure(user::config)
            .configure(video::config)
            .configure(danmaku::config)
    }).bind("0.0.0.0:8088")
    .unwrap()
    .run();
}
