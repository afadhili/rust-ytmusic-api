use serde_json::Value;

use crate::constants::PageType;
use crate::types::{HomeContent, HomeSection, ThumbnailFull};
use crate::utils::traverse::{traverse_first, traverse_list, traverse_string};

use super::{album::AlbumParser, playlist::PlaylistParser, song::SongParser};

pub struct Parser;

impl Parser {
    pub fn parse_duration(time: Option<&str>) -> Option<u64> {
        let time = time?.trim();
        if time.is_empty() {
            return None;
        }

        let parts = time
            .split(':')
            .rev()
            .filter_map(|n| n.parse::<u64>().ok())
            .collect::<Vec<_>>();

        let seconds = *parts.get(0).unwrap_or(&0);
        let minutes = *parts.get(1).unwrap_or(&0);
        let hours = *parts.get(2).unwrap_or(&0);
        Some(seconds + minutes * 60 + hours * 3600)
    }

    pub fn parse_number(value: &str) -> Option<u64> {
        let value = value.trim().replace(',', "");
        if value.is_empty() {
            return None;
        }
        let last = value.chars().last()?;
        if last.is_ascii_uppercase() {
            let number = value[..value.len() - 1].parse::<f64>().ok()?;
            let multiplier = match last {
                'K' => 1_000.0,
                'M' => 1_000_000.0,
                'B' => 1_000_000_000.0,
                'T' => 1_000_000_000_000.0,
                _ => return None,
            };
            Some((number * multiplier) as u64)
        } else {
            value.parse::<u64>().ok()
        }
    }

    pub fn parse_thumbnails(data: &Value, keys: &[&str]) -> Vec<ThumbnailFull> {
        fn push_thumbnail(item: &Value, out: &mut Vec<ThumbnailFull>) {
            if let Some(items) = item.as_array() {
                for item in items {
                    push_thumbnail(item, out);
                }
                return;
            }

            let Some(url) = item.get("url").and_then(Value::as_str) else {
                return;
            };

            out.push(ThumbnailFull {
                url: url.to_string(),
                width: item.get("width").and_then(Value::as_u64).unwrap_or(0) as u32,
                height: item.get("height").and_then(Value::as_u64).unwrap_or(0) as u32,
            });
        }

        let mut thumbnails = Vec::new();
        for item in traverse_list(data, keys) {
            push_thumbnail(item, &mut thumbnails);
        }
        thumbnails
    }

    pub fn parse_artist_basic(data: &Value, keys: &[&str]) -> crate::types::ArtistBasic {
        let root = traverse_first(data, keys).unwrap_or(data);
        let runs = traverse_list(root, &["runs"]);

        for run_group in runs {
            if let Some(run_items) = run_group.as_array() {
                for run in run_items {
                    let name = traverse_string(run, &["text"]);
                    let artist_id = traverse_string(run, &["browseId"]);

                    if !name.is_empty() && !artist_id.is_empty() {
                        return crate::types::ArtistBasic {
                            artist_id: Some(artist_id),
                            name,
                        };
                    }
                }
            }
        }

        crate::types::ArtistBasic {
            artist_id: Some(traverse_string(root, &["browseId"])).filter(|s| !s.is_empty()),
            name: traverse_string(root, &["text"]),
        }
    }

    pub fn parse_home_section(data: &Value) -> HomeSection {
        let page_type = traverse_string(data, &["contents", "title", "browseEndpoint", "pageType"]);
        let playlist_id = traverse_string(data, &["navigationEndpoint", "watchPlaylistEndpoint", "playlistId"]);

        let contents = traverse_list(data, &["contents"])
            .into_iter()
            .filter_map(|item| match PageType::from_str(&page_type) {
                Some(PageType::MusicPageTypeAlbum) => Some(HomeContent::Album(AlbumParser::parse_home_section(item))),
                Some(PageType::MusicPageTypePlaylist) => Some(HomeContent::Playlist(PlaylistParser::parse_home_section(item))),
                _ if !playlist_id.is_empty() => Some(HomeContent::Playlist(PlaylistParser::parse_home_section(item))),
                _ => Some(HomeContent::Song(SongParser::parse_home_section(item))),
            })
            .collect();

        HomeSection {
            title: traverse_string(data, &["header", "title", "text"]),
            contents,
        }
    }

    pub fn first_run<'a>(data: &'a Value, keys: &[&str]) -> Option<&'a Value> {
        traverse_first(data, keys)
    }
}
