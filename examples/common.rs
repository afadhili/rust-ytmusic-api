use anyhow::{Context, Result};
use serde::Serialize;
use ytmusic_rs::{InitializeOptions, MusicClient};

pub const DEFAULT_QUERY: &str = "Eminem";
pub const DEFAULT_SONG_QUERY: &str = "Lose Yourself";
pub const DEFAULT_VIDEO_ID: &str = "4wOLVrGHiIU";

pub async fn client() -> Result<MusicClient> {
    MusicClient::new()
        .init_with_options(InitializeOptions {
            cookies: std::env::var("YTMUSIC_COOKIES").ok(),
            gl: std::env::var("YTMUSIC_GL").ok().or_else(|| Some("US".to_string())),
            hl: std::env::var("YTMUSIC_HL").ok().or_else(|| Some("en".to_string())),
        })
        .await
        .context("failed to initialize MusicClient")
}

pub fn query() -> String {
    std::env::var("YTMUSIC_QUERY").unwrap_or_else(|_| DEFAULT_QUERY.to_string())
}

pub fn song_query() -> String {
    std::env::var("YTMUSIC_SONG_QUERY").unwrap_or_else(|_| DEFAULT_SONG_QUERY.to_string())
}

pub fn video_id() -> String {
    std::env::var("YTMUSIC_VIDEO_ID").unwrap_or_else(|_| DEFAULT_VIDEO_ID.to_string())
}

pub fn print_json<T: Serialize>(label: &str, value: &T) -> Result<()> {
    println!("\n=== {label} ===");
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
