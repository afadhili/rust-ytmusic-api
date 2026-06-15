# rs-ytmusic-api

A Rust client library for interacting with YouTube Music’s internal API.

This crate provides a lightweight interface for searching songs, videos, artists, albums, playlists, fetching song details, lyrics, artist data, album data, playlist tracks, home sections, and watch queue/up-next recommendations.

> This project is not affiliated with, endorsed by, or maintained by YouTube, Google, or YouTube Music.

## Why This Project Exists

I created this project because I wanted a Rust-native YouTube Music API client that can be used directly in terminal music players, TUI applications, personal tools, and other Rust projects.

Most existing YouTube Music API wrappers are written in JavaScript or Python. Since I am building a Rust-based music CLI/TUI, I wanted a library that fits naturally into the Rust ecosystem with:

- async/await support
- typed response models
- simple client API
- direct integration with Rust terminal applications
- minimal dependency on JavaScript tooling
- support for YouTube Music search, metadata, lyrics, playlists, and watch queue

This crate started as a Rust port and redesign inspired by existing YouTube Music API wrappers.

## Features

- Search YouTube Music
- Search songs
- Search videos
- Search artists
- Search albums
- Search playlists
- Fetch song details
- Fetch video details
- Fetch lyrics
- Fetch artist details
- Fetch artist songs
- Fetch artist albums
- Fetch album details
- Fetch playlist metadata
- Fetch playlist tracks
- Fetch home sections
- Fetch watch queue / up-next songs
- Raw search response helpers for debugging parser changes
- Region and language configuration
- Optional cookie support

## Basic Usage

```rust
use rs_ytmusic_api::MusicClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MusicClient::new().init().await?;

    let songs = client.find_songs("Eminem Lose Yourself").await?;

    for song in songs {
        println!("{} - {}", song.name, song.artist.name);
    }

    Ok(())
}
```

## Initialize Client

Basic initialization:

```rust
let client = MusicClient::new().init().await?;
```

Initialize with region and language:

```rust
use rs_ytmusic_api::{MusicClient, InitializeOptions};

let client = MusicClient::new()
    .init_with_options(InitializeOptions {
        gl: Some("US".to_string()),
        hl: Some("en".to_string()),
        cookies: None,
    })
    .await?;
```

Initialize with cookies:

```rust
use rs_ytmusic_api::{MusicClient, InitializeOptions};

let client = MusicClient::new()
    .init_with_options(InitializeOptions {
        gl: Some("US".to_string()),
        hl: Some("en".to_string()),
        cookies: Some("VISITOR_INFO1_LIVE=...; YSC=...".to_string()),
    })
    .await?;
```

## Search Suggestions

```rust
let suggestions = client.suggest("eminem").await?;

for suggestion in suggestions {
    println!("{suggestion}");
}
```

## Search All Result Types

```rust
use rs_ytmusic_api::{MusicClient, SearchResult};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MusicClient::new().init().await?;

    let results = client.find("Eminem").await?;

    for item in results {
        match item {
            SearchResult::Song(song) => {
                println!("Song: {} - {}", song.name, song.artist.name);
            }
            SearchResult::Video(video) => {
                println!("Video: {} - {}", video.name, video.artist.name);
            }
            SearchResult::Artist(artist) => {
                println!("Artist: {}", artist.name);
            }
            SearchResult::Album(album) => {
                println!("Album: {} - {}", album.name, album.artist.name);
            }
            SearchResult::Playlist(playlist) => {
                println!("Playlist: {}", playlist.name);
            }
        }
    }

    Ok(())
}
```

## Search Songs

```rust
let songs = client.find_songs("Lose Yourself").await?;

for song in songs {
    println!("{} - {}", song.name, song.artist.name);
}
```

## Search Videos

```rust
let videos = client.find_videos("Eminem live").await?;

for video in videos {
    println!("{} - {}", video.name, video.artist.name);
}
```

## Search Artists

```rust
let artists = client.find_artists("Eminem").await?;

for artist in artists {
    println!("{} ({})", artist.name, artist.artist_id);
}
```

## Search Albums

```rust
let albums = client.find_albums("The Eminem Show").await?;

for album in albums {
    println!("{} - {}", album.name, album.artist.name);
}
```

## Search Playlists

```rust
let playlists = client.find_playlists("Eminem playlist").await?;

for playlist in playlists {
    println!("{} ({})", playlist.name, playlist.playlist_id);
}
```

## Fetch Song Details

```rust
let song = client.fetch_song("4wOLVrGHiIU").await?;

println!("Title: {}", song.name);
println!("Artist: {}", song.artist.name);
println!("Duration: {} seconds", song.duration);
```

## Fetch Video Details

```rust
let video = client.fetch_video("4wOLVrGHiIU").await?;

println!("Title: {}", video.name);
println!("Artist: {}", video.artist.name);
```

## Fetch Watch Queue / Up Next

```rust
let queue = client.fetch_watch_queue("4wOLVrGHiIU").await?;

for item in queue {
    println!("{} - {}", item.title, item.artists.name);
}
```

You can also use the alias:

```rust
let queue = client.fetch_up_next("4wOLVrGHiIU").await?;
```

## Fetch Lyrics

```rust
let lyrics = client.fetch_lyrics("4wOLVrGHiIU").await?;

match lyrics {
    Some(lines) => {
        for line in lines {
            println!("{line}");
        }
    }
    None => {
        println!("Lyrics not found");
    }
}
```

## Fetch Artist Details

```rust
let artist = client.fetch_artist("UCedvOgsKFzcK3hA5taf3KoQ").await?;

println!("Artist: {}", artist.name);

for song in artist.top_songs {
    println!("Top song: {}", song.name);
}
```

## Fetch Artist Songs

```rust
let songs = client.fetch_artist_songs("UCedvOgsKFzcK3hA5taf3KoQ").await?;

for song in songs {
    println!("{} - {}", song.name, song.artist.name);
}
```

## Fetch Artist Albums

```rust
let albums = client.fetch_artist_albums("UCedvOgsKFzcK3hA5taf3KoQ").await?;

for album in albums {
    println!("{} ({:?})", album.name, album.year);
}
```

## Fetch Album Details

```rust
let album = client.fetch_album("MPREb_xxxxxxxx").await?;

println!("Album: {}", album.name);

for song in album.songs {
    println!("{} - {}", song.name, song.artist.name);
}
```

## Fetch Playlist Details

```rust
let playlist = client.fetch_playlist("PLxxxxxxxxxxxxxxxx").await?;

println!("Playlist: {}", playlist.name);
println!("Video count: {}", playlist.video_count);
```

## Fetch Playlist Tracks

```rust
let tracks = client.fetch_playlist_tracks("PLxxxxxxxxxxxxxxxx").await?;

for track in tracks {
    println!("{} - {}", track.name, track.artist.name);
}
```

## Fetch Home Sections

```rust
let sections = client.fetch_home().await?;

for section in sections {
    println!("Section: {}", section.title);
}
```

## Raw Debug Helpers

Use raw helpers when YouTube Music changes response shape and parsed results are empty.

```rust
let raw = client.raw_search("Eminem").await?;
std::fs::write("search_raw.json", serde_json::to_string_pretty(&raw)?)?;
```

Song search raw response:

```rust
let raw = client.raw_song_search("Eminem").await?;
std::fs::write("song_search_raw.json", serde_json::to_string_pretty(&raw)?)?;
```

## Common Types

The package exposes these common types:

```rust
MusicClient
InitializeOptions
SearchResult
SongDetailed
VideoDetailed
ArtistDetailed
AlbumDetailed
PlaylistDetailed
SongFull
VideoFull
ArtistFull
AlbumFull
PlaylistFull
UpNextDetails
HomeSection
ThumbnailFull
ArtistBasic
AlbumBasic
```

## Example: Simple Music Search

```rust
use rs_ytmusic_api::MusicClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MusicClient::new().init().await?;

    let songs = client.find_songs("Eminem Mockingbird").await?;

    if let Some(song) = songs.first() {
        println!("Video ID: {}", song.video_id);
        println!("Title: {}", song.name);
        println!("Artist: {}", song.artist.name);
        println!("Duration: {:?}", song.duration);
    }

    Ok(())
}
```

## Example: Build a Queue

```rust
use rs_ytmusic_api::MusicClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MusicClient::new().init().await?;

    let song = client.fetch_song("4wOLVrGHiIU").await?;
    println!("Now playing: {} - {}", song.name, song.artist.name);

    let up_next = client.fetch_watch_queue(&song.video_id).await?;

    for item in up_next {
        println!("Up next: {} - {}", item.title, item.artists.name);
    }

    Ok(())
}
```

## Notes

This package uses YouTube Music’s internal API. The API is not officially documented, so response structures may change at any time.

Some fields may be empty depending on region, language, content type, or YouTube Music response changes.

For production apps, always handle optional or empty fields safely.

## License

GPL-3.0

## Disclaimer

This project is unofficial.

YouTube Music is a trademark of Google LLC. This project is not affiliated with Google, YouTube, or YouTube Music.
