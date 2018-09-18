use actix_web::{server, App, HttpRequest};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod danmaku;
mod user;
mod video;

fn index(_req: &HttpRequest) -> &'static str {
    "Hello world!"
}

fn main() {
    //load ssl keys
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file("key.pem",SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

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
