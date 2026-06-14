mod common;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = common::client().await?;
    let albums = client.find_albums(&common::query()).await?;
    common::print_json("find_albums", &albums)
}
