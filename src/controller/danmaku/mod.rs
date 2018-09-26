use actix_web::{App, HttpResponse};

mod danmaku;

pub fn danmaku(app: App) -> App {
    app.prefix("/danmaku")
    .scope("/danmaku", |proj_scope| {
        proj_scope.resource("", |r| r.get().f(danmaku::index))
    })
    .scope("/test", |proj_scope| {
        proj_scope.resource("", |r| r.f(|_| HttpResponse::Ok()))
    })
    .scope("/test2", |proj_scope| {
        proj_scope.resource("", |r| r.get().f(danmaku::index))
    })
}