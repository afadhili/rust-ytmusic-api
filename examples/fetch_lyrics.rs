mod common;
use anyhow::Context;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = common::client().await?;
    let video_id = if let Ok(id) = std::env::var("YTMUSIC_VIDEO_ID") {
        id
    } else {
        client
            .find_songs(&common::song_query())
            .await?
            .into_iter()
            .next()
            .context("find_songs returned no songs")?
            .video_id
    };

    let lyrics = client.fetch_lyrics(&video_id).await?;
    common::print_json("fetch_lyrics", &lyrics)
}
