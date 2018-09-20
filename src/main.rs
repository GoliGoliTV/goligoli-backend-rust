use actix_web::{server, App, HttpRequest};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use log::trace;

mod danmaku;
mod user;
mod video;

fn index(_req: &HttpRequest) -> &'static str {
    "Hello world!"
}

fn main() {
    pretty_env_logger::init();

    dotenv::dotenv().ok();
    let private_key_file =
        dotenv::var("PRIVATE_KEY_FILE").expect("PRIVATE_KEY_FILE variable not set in .env file");
    let cert_file = dotenv::var("CERTIFICATE_CHAIN_FILE")
        .expect("CERTIFICATE_CHAIN_FILE variable not set in .env file");
    trace!(".env file loaded");

    // load ssl keys
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(private_key_file, SslFiletype::PEM)
        .expect("Fail to load private key file");
    builder
        .set_certificate_chain_file(cert_file)
        .expect("Fail to load cert file");

    server::new(|| {
        App::new()
            .resource("/", |r| r.f(index))
            .configure(user::config)
            .configure(video::config)
            .configure(danmaku::config)
    }).bind_ssl("127.0.0.1:8088", builder)
    .unwrap()
    .run();
}
