use actix_web::{App};

mod user;

pub fn user(app: App) -> App {
    app.prefix("/user")
    .resource("/signup", |r| r.post().f(user::user_signup))
    .resource("/signin", |r| r.post().f(user::user_signin))
    .scope("/{username}", |proj_scope|{
        proj_scope.resource("/home", |r| r.get().f(user::user_zone))
    })
}