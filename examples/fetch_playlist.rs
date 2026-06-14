mod common;
use anyhow::Context;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = common::client().await?;
    let playlist_id = if let Ok(id) = std::env::var("YTMUSIC_PLAYLIST_ID") {
        id
    } else {
        client
            .find_playlists(&common::query())
            .await?
            .into_iter()
            .next()
            .context("find_playlists returned no playlists")?
            .playlist_id
    };

    let playlist = client.fetch_playlist(&playlist_id).await?;
    common::print_json("fetch_playlist", &playlist)
}
