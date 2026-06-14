mod common;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = common::client().await?;
    let raw = client.raw_search(&common::query()).await?;
    common::print_json("raw_search", &raw)
}
