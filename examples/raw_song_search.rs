mod common;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = common::client().await?;
    let raw = client.raw_song_search(&common::song_query()).await?;
    common::print_json("raw_song_search", &raw)
}
