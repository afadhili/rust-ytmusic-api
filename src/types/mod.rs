use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ThumbnailFull {
    pub url: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArtistBasic {
    #[serde(rename = "artistId")]
    pub artist_id: Option<String>,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AlbumBasic {
    #[serde(rename = "albumId")]
    pub album_id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SongDetailed {
    #[serde(rename = "videoId")]
    pub video_id: String,
    pub name: String,
    pub artist: ArtistBasic,
    pub album: Option<AlbumBasic>,
    pub duration: Option<u64>,
    pub thumbnails: Vec<ThumbnailFull>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VideoDetailed {
    #[serde(rename = "videoId")]
    pub video_id: String,
    pub name: String,
    pub artist: ArtistBasic,
    pub duration: Option<u64>,
    pub thumbnails: Vec<ThumbnailFull>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArtistDetailed {
    #[serde(rename = "artistId")]
    pub artist_id: String,
    pub name: String,
    pub thumbnails: Vec<ThumbnailFull>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AlbumDetailed {
    #[serde(rename = "albumId")]
    pub album_id: String,
    #[serde(rename = "playlistId")]
    pub playlist_id: String,
    pub name: String,
    pub artist: ArtistBasic,
    pub year: Option<u32>,
    pub thumbnails: Vec<ThumbnailFull>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlaylistDetailed {
    #[serde(rename = "playlistId")]
    pub playlist_id: String,
    pub name: String,
    pub artist: ArtistBasic,
    pub thumbnails: Vec<ThumbnailFull>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SongFull {
    #[serde(rename = "videoId")]
    pub video_id: String,
    pub name: String,
    pub artist: ArtistBasic,
    pub duration: u64,
    pub thumbnails: Vec<ThumbnailFull>,
    pub formats: Vec<serde_json::Value>,
    #[serde(rename = "adaptiveFormats")]
    pub adaptive_formats: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VideoFull {
    #[serde(rename = "videoId")]
    pub video_id: String,
    pub name: String,
    pub artist: ArtistBasic,
    pub duration: u64,
    pub thumbnails: Vec<ThumbnailFull>,
    pub unlisted: bool,
    #[serde(rename = "familySafe")]
    pub family_safe: bool,
    pub paid: bool,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpNextDetails {
    #[serde(rename = "videoId")]
    pub video_id: String,
    pub title: String,
    pub artists: ArtistBasic,
    pub duration: Option<u64>,
    pub thumbnails: Vec<ThumbnailFull>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArtistFull {
    #[serde(rename = "artistId")]
    pub artist_id: String,
    pub name: String,
    pub thumbnails: Vec<ThumbnailFull>,
    #[serde(rename = "topSongs")]
    pub top_songs: Vec<SongDetailed>,
    #[serde(rename = "topAlbums")]
    pub top_albums: Vec<AlbumDetailed>,
    #[serde(rename = "topSingles")]
    pub top_singles: Vec<AlbumDetailed>,
    #[serde(rename = "topVideos")]
    pub top_videos: Vec<VideoDetailed>,
    #[serde(rename = "featuredOn")]
    pub featured_on: Vec<PlaylistDetailed>,
    #[serde(rename = "similarArtists")]
    pub similar_artists: Vec<ArtistDetailed>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AlbumFull {
    #[serde(rename = "albumId")]
    pub album_id: String,
    #[serde(rename = "playlistId")]
    pub playlist_id: String,
    pub name: String,
    pub artist: ArtistBasic,
    pub year: Option<u32>,
    pub thumbnails: Vec<ThumbnailFull>,
    pub songs: Vec<SongDetailed>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlaylistFull {
    #[serde(rename = "playlistId")]
    pub playlist_id: String,
    pub name: String,
    pub artist: ArtistBasic,
    #[serde(rename = "videoCount")]
    pub video_count: u64,
    pub thumbnails: Vec<ThumbnailFull>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum SearchResult {
    #[serde(rename = "SONG")]
    Song(SongDetailed),
    #[serde(rename = "VIDEO")]
    Video(VideoDetailed),
    #[serde(rename = "ALBUM")]
    Album(AlbumDetailed),
    #[serde(rename = "ARTIST")]
    Artist(ArtistDetailed),
    #[serde(rename = "PLAYLIST")]
    Playlist(PlaylistDetailed),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HomeSection {
    pub title: String,
    pub contents: Vec<HomeContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum HomeContent {
    #[serde(rename = "SONG")]
    Song(SongDetailed),
    #[serde(rename = "ALBUM")]
    Album(AlbumDetailed),
    #[serde(rename = "PLAYLIST")]
    Playlist(PlaylistDetailed),
}
