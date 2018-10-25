#[macro_use]
extern crate actix;
extern crate bytes;
extern crate env_logger;
extern crate futures;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate json;

use actix_web::{middleware, server, App, HttpRequest};
use env_logger::Target;
use log::trace;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod controller;
mod db;
mod migrate;

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
    let private_key_file =
        dotenv::var("PRIVATE_KEY_FILE").expect("PRIVATE_KEY_FILE variable not set in .env file");
    let cert_file = dotenv::var("CERTIFICATE_CHAIN_FILE")
        .expect("CERTIFICATE_CHAIN_FILE variable not set in .env file");
    trace!(".env file loaded");

    //load db_controller
    db::db_controller();

    // load ssl keys
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(private_key_file, SslFiletype::PEM)
        .expect("Fail to load private key file");
    builder
        .set_certificate_chain_file(cert_file)
        .expect("Fail to load cert file");

    let sys = server::new(move || {
        vec![
            App::new()
                .middleware(middleware::Logger::new("%a %s %r %{User-Agent}i"))
                .middleware(migrate::SayHi)
                .configure(controller::danmaku::danmaku)
                .finish(),
            App::new()
                .middleware(middleware::Logger::new("%a %s %r %{User-Agent}i"))
                .middleware(migrate::SayHi)
                .configure(controller::user::user)
                .finish(),
            App::new()
                .middleware(middleware::Logger::new("%a %s %r %{User-Agent}i"))
                .middleware(migrate::SayHi)
                .configure(controller::video::video)
                .finish(),
        ]
    })
    .workers(4)
    .bind_ssl("127.0.0.1:8088", builder)
    .unwrap();

    println!("started http server: 127.0.0.1:8088");
    let _ = sys.run();
}

// fn setup_logger() -> Result<(), fern:::InitError> {

// }
