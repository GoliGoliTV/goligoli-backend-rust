extern crate actix_web;

use actix_web::middleware::{Finished, Middleware, Response, Started};
use actix_web::{HttpRequest, HttpResponse, Result};

pub struct SayHi;

impl<S> Middleware<S> for SayHi {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        println!("Hi from start. You requested: {}", req.path());
        Ok(Started::Done)
    }

    fn response(&self, _req: &HttpRequest<S>, resp: HttpResponse) -> Result<Response> {
        println!("Hi from response");
        Ok(Response::Done(resp))
    }

    fn finish(&self, _req: &HttpRequest<S>, _resp: &HttpResponse) -> Finished {
        println!("Hi from finish");
        Finished::Done
    }
}
