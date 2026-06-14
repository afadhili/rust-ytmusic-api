use serde_json::Value;

use crate::types::{ArtistBasic, VideoDetailed, VideoFull};
use crate::utils::filters::{is_artist, is_duration, is_title};
use crate::utils::traverse::{strings, traverse_list, traverse_string};

use super::common::Parser;

pub struct VideoParser;

impl VideoParser {
    pub fn parse(data: &Value) -> VideoFull {
        VideoFull {
            video_id: traverse_string(data, &["videoDetails", "videoId"]),
            name: traverse_string(data, &["videoDetails", "title"]),
            artist: ArtistBasic {
                artist_id: Some(traverse_string(data, &["videoDetails", "channelId"]))
                    .filter(|s| !s.is_empty()),
                name: traverse_string(data, &["author"]),
            },
            duration: traverse_string(data, &["videoDetails", "lengthSeconds"])
                .parse()
                .unwrap_or(0),
            thumbnails: Parser::parse_thumbnails(data, &["videoDetails", "thumbnails"]),
            unlisted: traverse_list(data, &["unlisted"])
                .first()
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
            family_safe: traverse_list(data, &["familySafe"])
                .first()
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
            paid: traverse_list(data, &["paid"])
                .first()
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
            tags: strings(data, &["tags"]),
        }
    }

    pub fn parse_search_result(item: &Value) -> VideoDetailed {
        let columns = traverse_list(item, &["flexColumns", "runs"]);
        let flat = columns
            .iter()
            .flat_map(|v| match v {
                Value::Array(a) => a.iter().collect::<Vec<_>>(),
                _ => vec![*v],
            })
            .collect::<Vec<_>>();
        let title = flat.iter().copied().find(|v| is_title(v)).unwrap_or(item);
        let artist = flat
            .iter()
            .copied()
            .find(|v| is_artist(v))
            .or_else(|| flat.get(1).copied())
            .unwrap_or(item);
        let duration = flat.iter().copied().find(|v| is_duration(v));

        VideoDetailed {
            video_id: traverse_string(item, &["playNavigationEndpoint", "videoId"]),
            name: traverse_string(title, &["text"]),
            artist: ArtistBasic {
                artist_id: Some(traverse_string(artist, &["browseId"])).filter(|s| !s.is_empty()),
                name: traverse_string(artist, &["text"]),
            },
            duration: Parser::parse_duration(
                duration.and_then(|d| d.get("text")).and_then(Value::as_str),
            ),
            thumbnails: Parser::parse_thumbnails(item, &["thumbnails"]),
        }
    }

    pub fn parse_artist_top_video(item: &Value, artist: ArtistBasic) -> VideoDetailed {
        VideoDetailed {
            video_id: traverse_string(item, &["videoId"]),
            name: traverse_string(item, &["runs", "text"]),
            artist,
            duration: None,
            thumbnails: Parser::parse_thumbnails(item, &["thumbnails"]),
        }
    }

    pub fn parse_playlist_video(item: &Value) -> Option<VideoDetailed> {
        let flex = traverse_list(item, &["flexColumns", "runs"]);
        let fixed = traverse_list(item, &["fixedColumns", "runs"]);
        let flat = flex
            .iter()
            .flat_map(|v| match v {
                Value::Array(a) => a.iter().collect::<Vec<_>>(),
                _ => vec![*v],
            })
            .collect::<Vec<_>>();
        let title = flat
            .iter()
            .copied()
            .find(|v| is_title(v))
            .or_else(|| flat.get(0).copied())
            .unwrap_or(item);
        let artist = flat
            .iter()
            .copied()
            .find(|v| is_artist(v))
            .or_else(|| flat.get(1).copied())
            .unwrap_or(item);
        let duration = fixed.iter().copied().find(|v| is_duration(v));
        let mut video_id = traverse_string(item, &["playNavigationEndpoint", "videoId"]);
        if video_id.is_empty() {
            if let Some(url) = Parser::parse_thumbnails(item, &["thumbnails"])
                .first()
                .map(|t| t.url.clone())
            {
                if let Some(id) = url.split("/vi/").nth(1).and_then(|s| s.split('/').next()) {
                    video_id = id.to_string();
                }
            }
        }
        if video_id.is_empty() {
            return None;
        }
        Some(VideoDetailed {
            video_id,
            name: traverse_string(title, &["text"]),
            artist: ArtistBasic {
                name: traverse_string(artist, &["text"]),
                artist_id: Some(traverse_string(artist, &["browseId"])).filter(|s| !s.is_empty()),
            },
            duration: Parser::parse_duration(
                duration.and_then(|d| d.get("text")).and_then(Value::as_str),
            ),
            thumbnails: Parser::parse_thumbnails(item, &["thumbnails"]),
        })
    }
}
