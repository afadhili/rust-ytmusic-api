mod common;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = common::client().await?;
    let videos = client.find_videos(&common::query()).await?;
    common::print_json("find_videos", &videos)
}
