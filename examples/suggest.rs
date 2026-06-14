mod common;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = common::client().await?;
    let suggestions = client.suggest(&common::query()).await?;
    common::print_json("suggest", &suggestions)
}
