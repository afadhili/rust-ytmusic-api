use ytmusic_rs::{InitializeOptions, MusicClient};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MusicClient::new()
        .init_with_options(InitializeOptions {
            cookies: std::env::var("YTMUSIC_COOKIES").ok(),
            gl: Some(std::env::var("YTMUSIC_GL").unwrap_or_else(|_| "US".to_string())),
            hl: Some(std::env::var("YTMUSIC_HL").unwrap_or_else(|_| "en".to_string())),
        })
        .await?;

    println!("initialized with options: {client:#?}");
    Ok(())
}
