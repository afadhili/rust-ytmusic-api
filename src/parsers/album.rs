use serde_json::Value;

use crate::types::{AlbumBasic, AlbumDetailed, AlbumFull, ArtistBasic};
use crate::utils::filters::is_artist;
use crate::utils::traverse::{traverse_first, traverse_list, traverse_string};

use super::{common::Parser, song::SongParser};

pub struct AlbumParser;

impl AlbumParser {
    pub fn parse(data: &Value, album_id: &str) -> AlbumFull {
        let album = AlbumBasic {
            album_id: album_id.to_string(),
            name: traverse_string(data, &["tabs", "title", "text"]),
        };
        let artist_data =
            traverse_first(data, &["tabs", "straplineTextOne", "runs"]).unwrap_or(data);
        let artist = ArtistBasic {
            artist_id: Some(traverse_string(artist_data, &["browseId"])).filter(|s| !s.is_empty()),
            name: traverse_string(artist_data, &["text"]),
        };
        let thumbnails = Parser::parse_thumbnails(data, &["background", "thumbnails"]);
        let songs = traverse_list(data, &["musicResponsiveListItemRenderer"])
            .into_iter()
            .map(|item| {
                SongParser::parse_album_song(
                    item,
                    artist.clone(),
                    album.clone(),
                    thumbnails.clone(),
                )
            })
            .collect();
        AlbumFull {
            album_id: album.album_id,
            playlist_id: traverse_string(data, &["musicPlayButtonRenderer", "playlistId"]),
            name: album.name,
            artist,
            year: Self::process_year(
                traverse_list(data, &["tabs", "subtitle", "text"])
                    .last()
                    .and_then(|v| v.as_str()),
            ),
            thumbnails,
            songs,
        }
    }

    pub fn parse_search_result(item: &Value) -> AlbumDetailed {
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
        let playlist_id = {
            let a = traverse_string(item, &["overlay", "playlistId"]);
            if a.is_empty() {
                traverse_string(item, &["thumbnailOverlay", "playlistId"])
            } else {
                a
            }
        };
        AlbumDetailed {
            album_id: traverse_list(item, &["browseId"])
                .last()
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            playlist_id,
            artist: ArtistBasic {
                name: traverse_string(artist, &["text"]),
                artist_id: Some(traverse_string(artist, &["browseId"])).filter(|s| !s.is_empty()),
            },
            year: Self::process_year(
                flat.last()
                    .and_then(|v| v.get("text"))
                    .and_then(Value::as_str),
            ),
            name: traverse_string(title, &["text"]),
            thumbnails: Parser::parse_thumbnails(item, &["thumbnails"]),
        }
    }

    pub fn parse_artist_album(item: &Value, artist: ArtistBasic) -> AlbumDetailed {
        AlbumDetailed {
            album_id: traverse_list(item, &["browseId"])
                .last()
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            playlist_id: traverse_string(item, &["thumbnailOverlay", "playlistId"]),
            name: traverse_string(item, &["title", "text"]),
            artist,
            year: Self::process_year(
                traverse_list(item, &["subtitle", "text"])
                    .last()
                    .and_then(|v| v.as_str()),
            ),
            thumbnails: Parser::parse_thumbnails(item, &["thumbnails"]),
        }
    }

    pub fn parse_artist_top_album(item: &Value, artist: ArtistBasic) -> AlbumDetailed {
        AlbumDetailed {
            album_id: traverse_list(item, &["browseId"])
                .last()
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            playlist_id: traverse_string(item, &["musicPlayButtonRenderer", "playlistId"]),
            name: traverse_string(item, &["title", "text"]),
            artist,
            year: Self::process_year(
                traverse_list(item, &["subtitle", "text"])
                    .last()
                    .and_then(|v| v.as_str()),
            ),
            thumbnails: Parser::parse_thumbnails(item, &["thumbnails"]),
        }
    }

    pub fn parse_home_section(item: &Value) -> AlbumDetailed {
        let artist = traverse_first(item, &["subtitle", "runs"]).unwrap_or(item);
        AlbumDetailed {
            album_id: traverse_string(item, &["title", "browseId"]),
            playlist_id: traverse_string(item, &["thumbnailOverlay", "playlistId"]),
            name: traverse_string(item, &["title", "text"]),
            artist: ArtistBasic {
                name: traverse_string(artist, &["text"]),
                artist_id: Some(traverse_string(artist, &["browseId"])).filter(|s| !s.is_empty()),
            },
            year: None,
            thumbnails: Parser::parse_thumbnails(item, &["thumbnails"]),
        }
    }

    fn process_year(year: Option<&str>) -> Option<u32> {
        let y = year?;
        if y.len() == 4 && y.chars().all(|c| c.is_ascii_digit()) {
            y.parse().ok()
        } else {
            None
        }
    }
}
