mod common;
use anyhow::Context;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = common::client().await?;
    let artist_id = if let Ok(id) = std::env::var("YTMUSIC_ARTIST_ID") {
        id
    } else {
        client
            .find_artists(&common::query())
            .await?
            .into_iter()
            .next()
            .context("find_artists returned no artists")?
            .artist_id
    };

    let songs = client.fetch_artist_songs(&artist_id).await?;
    common::print_json("fetch_artist_songs", &songs)
}
