use actix_web::{App, HttpResponse};

mod danmaku;

pub fn danmaku(app: App) -> App {
    app.prefix("/danmaku")
    .scope("/danmaku", |proj_scope| {
        proj_scope.resource("", |r| r.get().f(danmaku::index))
    })
    .scope("/getdanmakubyvideo", |proj_scope| {
        proj_scope.resource("", |r| r.get().f(danmaku::get_danmaku_by_video))
    })
    .scope("/senddanmaku", |proj_scope| {
        proj_scope.resource("", |r| r.post().f(danmaku::send_danmaku))
    })
}