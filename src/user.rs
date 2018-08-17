use actix_web::{App, HttpRequest};

pub fn config(app: App) -> App {
    app.scope("/user", |proj_scope| {
        proj_scope.resource("", |r| r.get().f(user_test))
    })
}

fn user_test(_req: &HttpRequest) -> &'static str {
    "User Hello world!"
}
