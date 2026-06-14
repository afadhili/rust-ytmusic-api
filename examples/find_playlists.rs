mod common;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = common::client().await?;
    let playlists = client.find_playlists(&common::query()).await?;
    common::print_json("find_playlists", &playlists)
}
