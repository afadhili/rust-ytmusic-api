mod common;
use anyhow::Context;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = common::client().await?;
    let album_id = if let Ok(id) = std::env::var("YTMUSIC_ALBUM_ID") {
        id
    } else {
        client
            .find_albums(&common::query())
            .await?
            .into_iter()
            .next()
            .context("find_albums returned no albums")?
            .album_id
    };

    let album = client.fetch_album(&album_id).await?;
    common::print_json("fetch_album", &album)
}
