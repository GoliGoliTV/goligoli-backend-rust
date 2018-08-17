use actix_web::{App, HttpRequest};

pub fn config(app: App) -> App {
    app.scope("/video", |proj_scope| {
        // proj_scope.resource("", |r| r.get().f(user_test))
    })
}
