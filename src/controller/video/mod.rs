use actix_web::{App};

mod video;

pub fn video(app: App) -> App {
    app.scope("/video", |proj_scope| {
        proj_scope.resource("", |r| r.get().f(video::index))
    })
}