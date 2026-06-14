use serde_json::Value;

use crate::types::{ArtistBasic, ArtistDetailed, ArtistFull};
use crate::utils::traverse::{traverse_list, traverse_string};

use super::{
    album::AlbumParser, common::Parser, playlist::PlaylistParser, song::SongParser,
    video::VideoParser,
};

pub struct ArtistParser;

impl ArtistParser {
    pub fn parse(data: &Value, artist_id: &str) -> ArtistFull {
        let artist = ArtistBasic {
            artist_id: Some(artist_id.to_string()),
            name: traverse_string(data, &["header", "title", "text"]),
        };
        let carousels = traverse_list(data, &["musicCarouselShelfRenderer"]);
        ArtistFull {
            artist_id: artist_id.to_string(),
            name: artist.name.clone(),
            thumbnails: Parser::parse_thumbnails(data, &["header", "thumbnails"]),
            top_songs: traverse_list(data, &["musicShelfRenderer", "contents"])
                .into_iter()
                .map(|item| SongParser::parse_artist_top_song(item, artist.clone()))
                .collect(),
            top_albums: carousels
                .get(0)
                .and_then(|v| v.get("contents"))
                .and_then(Value::as_array)
                .map(|a| {
                    a.iter()
                        .map(|i| AlbumParser::parse_artist_top_album(i, artist.clone()))
                        .collect()
                })
                .unwrap_or_default(),
            top_singles: carousels
                .get(1)
                .and_then(|v| v.get("contents"))
                .and_then(Value::as_array)
                .map(|a| {
                    a.iter()
                        .map(|i| AlbumParser::parse_artist_top_album(i, artist.clone()))
                        .collect()
                })
                .unwrap_or_default(),
            top_videos: carousels
                .get(2)
                .and_then(|v| v.get("contents"))
                .and_then(Value::as_array)
                .map(|a| {
                    a.iter()
                        .map(|i| VideoParser::parse_artist_top_video(i, artist.clone()))
                        .collect()
                })
                .unwrap_or_default(),
            featured_on: carousels
                .get(3)
                .and_then(|v| v.get("contents"))
                .and_then(Value::as_array)
                .map(|a| {
                    a.iter()
                        .map(|i| PlaylistParser::parse_artist_featured_on(i, artist.clone()))
                        .collect()
                })
                .unwrap_or_default(),
            similar_artists: carousels
                .get(4)
                .and_then(|v| v.get("contents"))
                .and_then(Value::as_array)
                .map(|a| a.iter().map(Self::parse_similar_artist).collect())
                .unwrap_or_default(),
        }
    }

    pub fn parse_search_result(item: &Value) -> ArtistDetailed {
        let columns = traverse_list(item, &["flexColumns", "runs"]);
        let flat = columns
            .iter()
            .flat_map(|v| match v {
                Value::Array(a) => a.iter().collect::<Vec<_>>(),
                _ => vec![*v],
            })
            .collect::<Vec<_>>();
        let title = flat.get(0).copied().unwrap_or(item);
        ArtistDetailed {
            artist_id: traverse_string(item, &["browseId"]),
            name: traverse_string(title, &["text"]),
            thumbnails: Parser::parse_thumbnails(item, &["thumbnails"]),
        }
    }

    pub fn parse_similar_artist(item: &Value) -> ArtistDetailed {
        ArtistDetailed {
            artist_id: traverse_string(item, &["browseId"]),
            name: traverse_string(item, &["runs", "text"]),
            thumbnails: Parser::parse_thumbnails(item, &["thumbnails"]),
        }
    }
}
