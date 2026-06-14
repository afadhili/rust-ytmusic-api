use serde_json::Value;

use crate::types::{AlbumBasic, ArtistBasic, SongDetailed, SongFull, ThumbnailFull};
use crate::utils::filters::{is_album, is_artist, is_duration, is_title};
use crate::utils::traverse::{traverse_list, traverse_string};

use super::common::Parser;

pub struct SongParser;

impl SongParser {
    pub fn parse(data: &Value) -> SongFull {
        SongFull {
            video_id: traverse_string(data, &["videoDetails", "videoId"]),
            name: traverse_string(data, &["videoDetails", "title"]),
            artist: ArtistBasic {
                name: traverse_string(data, &["author"]),
                artist_id: Some(traverse_string(data, &["videoDetails", "channelId"]))
                    .filter(|s| !s.is_empty()),
            },
            duration: traverse_string(data, &["videoDetails", "lengthSeconds"])
                .parse()
                .unwrap_or(0),
            thumbnails: Parser::parse_thumbnails(data, &["videoDetails", "thumbnails"]),
            formats: traverse_list(data, &["streamingData", "formats"])
                .into_iter()
                .cloned()
                .collect(),
            adaptive_formats: traverse_list(data, &["streamingData", "adaptiveFormats"])
                .into_iter()
                .cloned()
                .collect(),
        }
    }

    pub fn parse_search_result(item: &Value) -> SongDetailed {
        let columns = traverse_list(item, &["flexColumns", "runs"]);
        let flat = columns
            .iter()
            .flat_map(|v| match v {
                Value::Array(a) => a.iter().collect::<Vec<_>>(),
                _ => vec![*v],
            })
            .collect::<Vec<_>>();
        let title = flat
            .get(0)
            .copied()
            .or(columns.get(0).copied())
            .unwrap_or(item);
        let artist = flat
            .iter()
            .copied()
            .find(|v| is_artist(v))
            .or_else(|| flat.get(3).copied())
            .unwrap_or(item);
        let album = flat.iter().copied().find(|v| is_album(v));
        let duration = flat.iter().copied().find(|v| is_duration(v));

        SongDetailed {
            video_id: traverse_string(item, &["playlistItemData", "videoId"]),
            name: traverse_string(title, &["text"]),
            artist: ArtistBasic {
                name: traverse_string(artist, &["text"]),
                artist_id: Some(traverse_string(artist, &["browseId"])).filter(|s| !s.is_empty()),
            },
            album: album.map(|a| AlbumBasic {
                name: traverse_string(a, &["text"]),
                album_id: traverse_string(a, &["browseId"]),
            }),
            duration: Parser::parse_duration(
                duration.and_then(|d| d.get("text")).and_then(Value::as_str),
            ),
            thumbnails: Parser::parse_thumbnails(item, &["thumbnails"]),
        }
    }

    pub fn parse_artist_song(item: &Value, artist: ArtistBasic) -> SongDetailed {
        let columns = traverse_list(item, &["flexColumns", "runs"]);
        let flat = columns
            .iter()
            .flat_map(|v| match v {
                Value::Array(a) => a.iter().collect::<Vec<_>>(),
                _ => vec![*v],
            })
            .collect::<Vec<_>>();
        let title = flat.iter().copied().find(|v| is_title(v)).unwrap_or(item);
        let album = flat.iter().copied().find(|v| is_album(v));
        let duration = flat.iter().copied().find(|v| is_duration(v));

        SongDetailed {
            video_id: traverse_string(item, &["playlistItemData", "videoId"]),
            name: traverse_string(title, &["text"]),
            artist,
            album: album.map(|a| AlbumBasic {
                name: traverse_string(a, &["text"]),
                album_id: traverse_string(a, &["browseId"]),
            }),
            duration: Parser::parse_duration(
                duration.and_then(|d| d.get("text")).and_then(Value::as_str),
            ),
            thumbnails: Parser::parse_thumbnails(item, &["thumbnails"]),
        }
    }

    pub fn parse_artist_top_song(item: &Value, artist: ArtistBasic) -> SongDetailed {
        let columns = traverse_list(item, &["flexColumns", "runs"]);
        let flat = columns
            .iter()
            .flat_map(|v| match v {
                Value::Array(a) => a.iter().collect::<Vec<_>>(),
                _ => vec![*v],
            })
            .collect::<Vec<_>>();
        let title = flat.iter().copied().find(|v| is_title(v)).unwrap_or(item);
        let album = flat.iter().copied().find(|v| is_album(v)).unwrap_or(item);

        SongDetailed {
            video_id: traverse_string(item, &["playlistItemData", "videoId"]),
            name: traverse_string(title, &["text"]),
            artist,
            album: Some(AlbumBasic {
                name: traverse_string(album, &["text"]),
                album_id: traverse_string(album, &["browseId"]),
            }),
            duration: None,
            thumbnails: Parser::parse_thumbnails(item, &["thumbnails"]),
        }
    }

    pub fn parse_album_song(
        item: &Value,
        artist: ArtistBasic,
        album: AlbumBasic,
        thumbnails: Vec<ThumbnailFull>,
    ) -> SongDetailed {
        let flex = traverse_list(item, &["flexColumns", "runs"]);
        let fixed = traverse_list(item, &["fixedColumns", "runs"]);
        let title = flex.iter().copied().find(|v| is_title(v)).unwrap_or(item);
        let duration = fixed.iter().copied().find(|v| is_duration(v));

        SongDetailed {
            video_id: traverse_string(item, &["playlistItemData", "videoId"]),
            name: traverse_string(title, &["text"]),
            artist,
            album: Some(album),
            duration: Parser::parse_duration(
                duration.and_then(|d| d.get("text")).and_then(Value::as_str),
            ),
            thumbnails,
        }
    }

    pub fn parse_home_section(item: &Value) -> SongDetailed {
        Self::parse_search_result(item)
    }
}
