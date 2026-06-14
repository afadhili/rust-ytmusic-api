mod common;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = common::client().await?;
    let songs = client.find_songs(&common::song_query()).await?;
    common::print_json("find_songs", &songs)
}
