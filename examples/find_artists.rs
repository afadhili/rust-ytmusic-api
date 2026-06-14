mod common;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = common::client().await?;
    let artists = client.find_artists(&common::query()).await?;
    common::print_json("find_artists", &artists)
}
