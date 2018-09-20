use actix_web::{server, App, HttpRequest};
use env_logger::Target;
use log::trace;

mod danmaku;
mod user;
mod video;

fn index(_req: &HttpRequest) -> &'static str {
    "Hello world!"
}

fn main() {
    let mut log_builder = pretty_env_logger::formatted_builder().unwrap();
    // let's just set some random stuff.. for more see
    // https://docs.rs/env_logger/0.5.0-rc.1/env_logger/struct.Builder.html
    log_builder.target(Target::Stdout);
    if let Ok(s) = ::std::env::var("RUST_LOG") {
        log_builder.parse(&s);
    };
    log_builder.init();

    dotenv::dotenv().ok();
    trace!(".env file loaded");

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
