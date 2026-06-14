mod common;
use anyhow::{bail, Context};

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

    if video_id.len() != 11 { bail!("video id should usually be 11 chars, got {video_id}"); }
    let song = client.fetch_song(&video_id).await?;
    common::print_json("fetch_song", &song)
}
