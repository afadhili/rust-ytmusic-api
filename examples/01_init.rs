use ytmusic_rs::MusicClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MusicClient::new().init().await?;
    println!("initialized: {client:#?}");
    Ok(())
}
