use actix_web::{App, HttpRequest, HttpResponse};

pub fn danmaku(app: App) -> App {
    app.prefix("/danmaku")
    .scope("/danmaku", |proj_scope| {
        proj_scope.resource("", |r| r.get().f(index))
    })
    .scope("/test", |proj_scope| {
        proj_scope.resource("", |r| r.f(|_| HttpResponse::Ok()))
    })
    .scope("/test2", |proj_scope| {
        proj_scope.resource("", |r| r.get().f(index))
    })
}

pub fn user(app: App) -> App {
    app.scope("/user", |proj_scope| {
        proj_scope.resource("", |r| r.get().f(user_test))
    })
}

pub fn video(app: App) -> App {
    app.scope("/video", |proj_scope| {
        proj_scope.resource("", |r| r.get().f(index))
    })
}

fn index(_req: &HttpRequest) -> &'static str {
    "Hello world!"
}

fn user_test(_req: &HttpRequest) -> &'static str {
    "User Hello world!"
}