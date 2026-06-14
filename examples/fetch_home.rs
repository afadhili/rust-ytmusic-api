mod common;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = common::client().await?;
    let sections = client.fetch_home().await?;
    common::print_json("fetch_home", &sections)
}
