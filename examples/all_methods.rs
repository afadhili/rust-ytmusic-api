mod common;
use anyhow::Context;
use ytmusic_rs::{InitializeOptions, MusicClient};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let query = common::query();
    let song_query = common::song_query();

    let _created = MusicClient::new();
    println!("MusicClient::new() OK");

    let _default_init = MusicClient::new().init().await?;
    println!("init() OK");

    let client = MusicClient::new()
        .init_with_options(InitializeOptions {
            cookies: std::env::var("YTMUSIC_COOKIES").ok(),
            gl: std::env::var("YTMUSIC_GL").ok().or_else(|| Some("US".to_string())),
            hl: std::env::var("YTMUSIC_HL").ok().or_else(|| Some("en".to_string())),
        })
        .await?;
    println!("init_with_options() OK");

    common::print_json("suggest", &client.suggest(&query).await?)?;

    let raw = client.raw_search(&query).await?;
    println!("raw_search OK: top-level keys = {}", raw.as_object().map(|m| m.len()).unwrap_or(0));

    let raw_songs = client.raw_song_search(&song_query).await?;
    println!("raw_song_search OK: top-level keys = {}", raw_songs.as_object().map(|m| m.len()).unwrap_or(0));

    common::print_json("find", &client.find(&query).await?)?;

    let songs = client.find_songs(&song_query).await?;
    common::print_json("find_songs", &songs)?;
    let song_id = std::env::var("YTMUSIC_VIDEO_ID")
        .ok()
        .or_else(|| songs.first().map(|s| s.video_id.clone()))
        .unwrap_or_else(common::video_id);

    common::print_json("find_videos", &client.find_videos(&query).await?)?;

    let artists = client.find_artists(&query).await?;
    common::print_json("find_artists", &artists)?;
    let artist_id = artists.first().context("find_artists returned no artists")?.artist_id.clone();

    let albums = client.find_albums(&query).await?;
    common::print_json("find_albums", &albums)?;
    let album_id = albums.first().context("find_albums returned no albums")?.album_id.clone();

    let playlists = client.find_playlists(&query).await?;
    common::print_json("find_playlists", &playlists)?;
    let playlist_id = playlists.first().context("find_playlists returned no playlists")?.playlist_id.clone();

    common::print_json("fetch_song", &client.fetch_song(&song_id).await?)?;
    common::print_json("fetch_video", &client.fetch_video(&song_id).await?)?;
    common::print_json("fetch_watch_queue", &client.fetch_watch_queue(&song_id).await?)?;
    common::print_json("fetch_lyrics", &client.fetch_lyrics(&song_id).await?)?;

    common::print_json("fetch_artist", &client.fetch_artist(&artist_id).await?)?;
    common::print_json("fetch_artist_songs", &client.fetch_artist_songs(&artist_id).await?)?;
    common::print_json("fetch_artist_albums", &client.fetch_artist_albums(&artist_id).await?)?;

    common::print_json("fetch_album", &client.fetch_album(&album_id).await?)?;
    common::print_json("fetch_playlist", &client.fetch_playlist(&playlist_id).await?)?;
    common::print_json("fetch_playlist_tracks", &client.fetch_playlist_tracks(&playlist_id).await?)?;
    common::print_json("fetch_home", &client.fetch_home().await?)?;

    Ok(())
}
