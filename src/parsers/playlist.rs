use serde_json::Value;

use crate::types::{ArtistBasic, PlaylistDetailed, PlaylistFull};
use crate::utils::filters::is_artist;
use crate::utils::traverse::{traverse_first, traverse_list, traverse_string};

use super::common::Parser;

pub struct PlaylistParser;

impl PlaylistParser {
    pub fn parse(data: &Value, playlist_id: &str) -> PlaylistFull {
        let artist = traverse_first(data, &["tabs", "straplineTextOne"]).unwrap_or(data);
        let count = traverse_list(data, &["tabs", "secondSubtitle", "text"])
            .get(2)
            .and_then(|v| v.as_str())
            .and_then(|s| s.split_whitespace().next())
            .and_then(Parser::parse_number)
            .unwrap_or(0);
        PlaylistFull {
            playlist_id: playlist_id.to_string(),
            name: traverse_string(data, &["tabs", "title", "text"]),
            artist: ArtistBasic {
                name: traverse_string(artist, &["text"]),
                artist_id: Some(traverse_string(artist, &["browseId"])).filter(|s| !s.is_empty()),
            },
            video_count: count,
            thumbnails: Parser::parse_thumbnails(data, &["tabs", "thumbnails"]),
        }
    }

    pub fn parse_search_result(item: &Value) -> PlaylistDetailed {
        let columns = traverse_list(item, &["flexColumns", "runs"]);
        let flat = columns
            .iter()
            .flat_map(|v| match v {
                Value::Array(a) => a.iter().collect::<Vec<_>>(),
                _ => vec![*v],
            })
            .collect::<Vec<_>>();
        let title = flat.get(0).copied().unwrap_or(item);
        let artist = flat
            .iter()
            .copied()
            .find(|v| is_artist(v))
            .or_else(|| flat.get(3).copied())
            .unwrap_or(item);
        PlaylistDetailed {
            playlist_id: traverse_string(item, &["overlay", "playlistId"]),
            name: traverse_string(title, &["text"]),
            artist: ArtistBasic {
                name: traverse_string(artist, &["text"]),
                artist_id: Some(traverse_string(artist, &["browseId"])).filter(|s| !s.is_empty()),
            },
            thumbnails: Parser::parse_thumbnails(item, &["thumbnails"]),
        }
    }

    pub fn parse_artist_featured_on(item: &Value, artist: ArtistBasic) -> PlaylistDetailed {
        PlaylistDetailed {
            playlist_id: traverse_string(item, &["navigationEndpoint", "browseId"]),
            name: traverse_string(item, &["runs", "text"]),
            artist,
            thumbnails: Parser::parse_thumbnails(item, &["thumbnails"]),
        }
    }

    pub fn parse_home_section(item: &Value) -> PlaylistDetailed {
        let artist = traverse_first(item, &["subtitle", "runs"]).unwrap_or(item);
        PlaylistDetailed {
            playlist_id: traverse_string(item, &["navigationEndpoint", "playlistId"]),
            name: traverse_string(item, &["runs", "text"]),
            artist: ArtistBasic {
                name: traverse_string(artist, &["text"]),
                artist_id: Some(traverse_string(artist, &["browseId"])).filter(|s| !s.is_empty()),
            },
            thumbnails: Parser::parse_thumbnails(item, &["thumbnails"]),
        }
    }
}
