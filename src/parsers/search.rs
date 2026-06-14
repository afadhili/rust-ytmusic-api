use serde_json::Value;

use crate::types::SearchResult;
use crate::utils::traverse::{traverse_list, traverse_string};

use super::{
    album::AlbumParser,
    artist::ArtistParser,
    playlist::PlaylistParser,
    song::SongParser,
    video::VideoParser,
};

pub struct SearchParser;

impl SearchParser {
    pub fn parse(item: &Value) -> Option<SearchResult> {
        let kind = Self::detect_kind(item)?;

        match kind.as_str() {
            "Song" => Some(SearchResult::Song(SongParser::parse_search_result(item))),
            "Video" => Some(SearchResult::Video(VideoParser::parse_search_result(item))),
            "Artist" => Some(SearchResult::Artist(ArtistParser::parse_search_result(item))),
            "EP" | "Single" | "Album" => Some(SearchResult::Album(AlbumParser::parse_search_result(item))),
            "Playlist" => Some(SearchResult::Playlist(PlaylistParser::parse_search_result(item))),
            _ => None,
        }
    }

    fn detect_kind(item: &Value) -> Option<String> {
        // Same strategy as the TS version: second flex column usually contains the type label.
        let flex_columns = traverse_list(item, &["flexColumns"]);
        if let Some(value) = flex_columns.get(1) {
            if let Some(kind) = Self::known_kind_from_texts(value) {
                return Some(kind);
            }
        }

        // Current YouTube responses are not always stable. Scan all visible text as fallback.
        if let Some(kind) = Self::known_kind_from_texts(item) {
            return Some(kind);
        }

        // Structural fallback. This also makes search() useful when labels are localized.
        let page_type = traverse_string(item, &["pageType"]);
        match page_type.as_str() {
            "MUSIC_PAGE_TYPE_ARTIST" | "MUSIC_PAGE_TYPE_USER_CHANNEL" => return Some("Artist".into()),
            "MUSIC_PAGE_TYPE_ALBUM" => return Some("Album".into()),
            "MUSIC_PAGE_TYPE_PLAYLIST" => return Some("Playlist".into()),
            _ => {}
        }

        let music_video_type = traverse_string(item, &["musicVideoType"]);
        if music_video_type == "MUSIC_VIDEO_TYPE_ATV" {
            return Some("Song".into());
        }
        if music_video_type.starts_with("MUSIC_VIDEO_TYPE_") {
            return Some("Video".into());
        }

        if !traverse_string(item, &["overlay", "playlistId"]).is_empty()
            || !traverse_string(item, &["thumbnailOverlay", "playlistId"]).is_empty()
        {
            return Some("Playlist".into());
        }

        if !traverse_string(item, &["playlistItemData", "videoId"]).is_empty() {
            return Some("Song".into());
        }
        if !traverse_string(item, &["playNavigationEndpoint", "videoId"]).is_empty() {
            return Some("Video".into());
        }

        None
    }

    fn known_kind_from_texts(data: &Value) -> Option<String> {
        for value in traverse_list(data, &["text"]) {
            let Some(text) = value.as_str() else { continue };
            match text {
                "Song" | "Video" | "Artist" | "EP" | "Single" | "Album" | "Playlist" => {
                    return Some(text.to_string())
                }
                _ => {}
            }
        }
        None
    }
}
