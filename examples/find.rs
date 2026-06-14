mod common;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = common::client().await?;
    let results = client.find(&common::query()).await?;
    common::print_json("find", &results)
}
