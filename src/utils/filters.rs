use regex::Regex;
use serde_json::Value;

use super::traverse::traverse_string;

pub fn is_title(data: &Value) -> bool {
    traverse_string(data, &["musicVideoType"]).starts_with("MUSIC_VIDEO_TYPE_")
}

pub fn is_artist(data: &Value) -> bool {
    matches!(
        traverse_string(data, &["pageType"]).as_str(),
        "MUSIC_PAGE_TYPE_USER_CHANNEL" | "MUSIC_PAGE_TYPE_ARTIST"
    )
}

pub fn is_album(data: &Value) -> bool {
    traverse_string(data, &["pageType"]) == "MUSIC_PAGE_TYPE_ALBUM"
}

pub fn is_duration(data: &Value) -> bool {
    let text = traverse_string(data, &["text"]);
    Regex::new(r"^(\d{1,2}:)?\d{1,2}:\d{1,2}$")
        .map(|re| re.is_match(&text))
        .unwrap_or(false)
}
