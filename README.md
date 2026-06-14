# rust-ytmusic-api

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
