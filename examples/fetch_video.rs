mod common;
use anyhow::Context;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = common::client().await?;
    let video_id = if let Ok(id) = std::env::var("YTMUSIC_VIDEO_ID") {
        id
    } else {
        client
            .find_videos(&common::query())
            .await?
            .into_iter()
            .next()
            .context("find_videos returned no videos")?
            .video_id
    };

    let video = client.fetch_video(&video_id).await?;
    common::print_json("fetch_video", &video)
}
