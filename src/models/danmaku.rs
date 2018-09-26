#[derive(Queryable)]

///## Danmaku data
/// 
pub struct Danmaku {
    pub ID:i32,
    pub VideoID:i64, 
    pub UID:i32, 
    pub content: String, 
}

//! NewDanmaku create new danmaku content
fn NewDanmaku(videoID:i64) -> &Danmaku{
    return &Danmaku{
        VideoID:videoID
    }
}