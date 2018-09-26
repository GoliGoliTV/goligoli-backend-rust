use actix_web::{App};

mod user;

pub fn user(app: App) -> App {
    app.scope("/user", |proj_scope| {
        proj_scope.resource("", |r| r.get().f(user::user_test))
    })
}