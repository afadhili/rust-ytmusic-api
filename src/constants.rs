#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageType {
    MusicPageTypeAlbum,
    MusicPageTypePlaylist,
    MusicVideoTypeOmv,
}

impl PageType {
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "MUSIC_PAGE_TYPE_ALBUM" => Some(Self::MusicPageTypeAlbum),
            "MUSIC_PAGE_TYPE_PLAYLIST" => Some(Self::MusicPageTypePlaylist),
            "MUSIC_VIDEO_TYPE_OMV" => Some(Self::MusicVideoTypeOmv),
            _ => None,
        }
    }
}

pub const FE_MUSIC_HOME: &str = "FEmusic_home";
