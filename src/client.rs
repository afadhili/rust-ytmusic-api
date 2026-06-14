use std::collections::HashMap;

use regex::Regex;
use reqwest::{header, Client};
use serde_json::{json, Map, Value};
use url::form_urlencoded;

use crate::constants::FE_MUSIC_HOME;
use crate::error::ClientError;
use crate::parsers::{album::AlbumParser, artist::ArtistParser, common::Parser, playlist::PlaylistParser, search::SearchParser, song::SongParser, video::VideoParser};
use crate::types::{AlbumDetailed, AlbumFull, ArtistDetailed, ArtistFull, HomeSection, PlaylistDetailed, PlaylistFull, SearchResult, SongDetailed, SongFull, UpNextDetails, VideoDetailed, VideoFull};
use crate::utils::traverse::{traverse_first, traverse_list, traverse_string};

#[derive(Debug, Clone, Default)]
pub struct InitializeOptions {
    pub cookies: Option<String>,
    pub gl: Option<String>,
    pub hl: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MusicClient {
    client: Client,
    config: Map<String, Value>,
    cookies: Option<String>,
}

impl Default for MusicClient {
    fn default() -> Self {
        Self::new()
    }
}

impl MusicClient {
    pub fn new() -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(header::USER_AGENT, header::HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.129 Safari/537.36"));
        headers.insert(header::ACCEPT_LANGUAGE, header::HeaderValue::from_static("en-US,en;q=0.5"));
        headers.insert(header::ACCEPT_ENCODING, header::HeaderValue::from_static("gzip"));

        let client = Client::builder()
            .default_headers(headers)
            .cookie_store(true)
            .build()
            .expect("failed to create reqwest client");

        Self { client, config: Map::new(), cookies: None }
    }

    pub async fn init(self) -> Result<Self, ClientError> {
        self.init_with_options(InitializeOptions::default()).await
    }

    pub async fn init_with_options(mut self, options: InitializeOptions) -> Result<Self, ClientError> {
        self.cookies = options.cookies;
        let html = self.client
            .get("https://music.youtube.com/")
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        let re = Regex::new(r#"ytcfg\.set\((\{.*?\})\);"#)?;
        for cap in re.captures_iter(&html) {
            if let Some(raw) = cap.get(1) {
                if let Ok(Value::Object(map)) = serde_json::from_str::<Value>(raw.as_str()) {
                    for (key, value) in map {
                        self.config.insert(key, value);
                    }
                }
            }
        }

        if let Some(gl) = options.gl { self.config.insert("GL".to_string(), Value::String(gl)); }
        if let Some(hl) = options.hl { self.config.insert("HL".to_string(), Value::String(hl)); }

        Ok(self)
    }

    fn config_str(&self, key: &'static str) -> Result<String, ClientError> {
        self.config
            .get(key)
            .and_then(Value::as_str)
            .map(ToString::to_string)
            .ok_or(ClientError::MissingConfig(key))
    }

    async fn construct_request(&self, endpoint: &str, body: Value, query: HashMap<String, String>) -> Result<Value, ClientError> {
        if self.config.is_empty() {
            return Err(ClientError::NotInitialized);
        }

        let api_version = self.config_str("INNERTUBE_API_VERSION")?;
        let api_key = self.config_str("INNERTUBE_API_KEY")?;
        let client_name = self.config_str("INNERTUBE_CLIENT_NAME")?;
        let client_version = self.config_str("INNERTUBE_CLIENT_VERSION")?;

        let mut params = form_urlencoded::Serializer::new(String::new());
        for (key, value) in query {
            params.append_pair(&key, &value);
        }
        params.append_pair("alt", "json");
        params.append_pair("key", &api_key);
        let params = params.finish();

        let url = format!("https://music.youtube.com/youtubei/{api_version}/{endpoint}?{params}");

        let gl = self.config.get("GL").and_then(Value::as_str).unwrap_or("US");
        let hl = self.config.get("HL").and_then(Value::as_str).unwrap_or("en");

        let mut req_body = json!({
            "context": {
                "capabilities": {},
                "client": {
                    "clientName": client_name,
                    "clientVersion": client_version,
                    "experimentIds": [],
                    "experimentsToken": "",
                    "gl": gl,
                    "hl": hl,
                    "locationInfo": {
                        "locationPermissionAuthorizationStatus": "LOCATION_PERMISSION_AUTHORIZATION_STATUS_UNSUPPORTED"
                    },
                    "musicAppInfo": {
                        "musicActivityMasterSwitch": "MUSIC_ACTIVITY_MASTER_SWITCH_INDETERMINATE",
                        "musicLocationMasterSwitch": "MUSIC_LOCATION_MASTER_SWITCH_INDETERMINATE",
                        "pwaInstallabilityStatus": "PWA_INSTALLABILITY_STATUS_UNKNOWN"
                    },
                    "utcOffsetMinutes": 0
                },
                "request": {
                    "internalExperimentFlags": [
                        { "key": "force_music_enable_outertube_tastebuilder_browse", "value": "true" },
                        { "key": "force_music_enable_outertube_playlist_detail_browse", "value": "true" },
                        { "key": "force_music_enable_outertube_search_suggestions", "value": "true" }
                    ],
                    "sessionIndex": {}
                },
                "user": { "enableSafetyMode": false }
            }
        });

        merge_json(&mut req_body, body);

        let mut request = self.client
            .post(url)
            .header("x-origin", "https://music.youtube.com/")
            .header("X-Goog-Visitor-Id", self.config.get("VISITOR_DATA").and_then(Value::as_str).unwrap_or(""))
            .header("X-YouTube-Client-Name", self.config.get("INNERTUBE_CONTEXT_CLIENT_NAME").and_then(Value::as_str).unwrap_or(""))
            .header("X-YouTube-Client-Version", self.config.get("INNERTUBE_CLIENT_VERSION").and_then(Value::as_str).unwrap_or(""))
            .header("X-YouTube-Device", self.config.get("DEVICE").and_then(Value::as_str).unwrap_or(""))
            .header("X-YouTube-Page-CL", self.config.get("PAGE_CL").and_then(Value::as_str).unwrap_or(""))
            .header("X-YouTube-Page-Label", self.config.get("PAGE_BUILD_LABEL").and_then(Value::as_str).unwrap_or(""))
            .header("X-YouTube-Utc-Offset", "0")
            .header("X-YouTube-Time-Zone", "UTC")
            .json(&req_body);

        if let Some(cookies) = &self.cookies {
            request = request.header(header::COOKIE, cookies);
        }

        Ok(request.send().await?.error_for_status()?.json::<Value>().await?)
    }

    pub async fn suggest(&self, query: &str) -> Result<Vec<String>, ClientError> {
        let data = self.construct_request("music/get_search_suggestions", json!({ "input": query }), HashMap::new()).await?;
        Ok(crate::utils::traverse::strings(&data, &["query"]))
    }

    pub async fn raw_search(&self, query: &str) -> Result<Value, ClientError> {
        self.construct_request("search", json!({ "query": query, "params": Value::Null }), HashMap::new()).await
    }

    pub async fn raw_song_search(&self, query: &str) -> Result<Value, ClientError> {
        self.construct_request("search", json!({ "query": query, "params": "Eg-KAQwIARAAGAAgACgAMABqChAEEAMQCRAFEAo%3D" }), HashMap::new()).await
    }

    pub async fn find(&self, query: &str) -> Result<Vec<SearchResult>, ClientError> {
        let data = self.construct_request("search", json!({ "query": query, "params": Value::Null }), HashMap::new()).await?;
        let items = traverse_list(&data, &["musicResponsiveListItemRenderer"]);
        let parsed: Vec<SearchResult> = items.into_iter().filter_map(SearchParser::parse).collect();
        if !parsed.is_empty() {
            return Ok(parsed);
        }

        // Fallback: some YouTube Music search responses omit/relocate the general type labels.
        // Typed endpoints are more stable, so find() still returns useful results.
        let mut results = Vec::new();
        results.extend(self.find_songs(query).await?.into_iter().map(SearchResult::Song));
        results.extend(self.find_videos(query).await?.into_iter().map(SearchResult::Video));
        results.extend(self.find_artists(query).await?.into_iter().map(SearchResult::Artist));
        results.extend(self.find_albums(query).await?.into_iter().map(SearchResult::Album));
        results.extend(self.find_playlists(query).await?.into_iter().map(SearchResult::Playlist));
        Ok(results)
    }

    pub async fn find_songs(&self, query: &str) -> Result<Vec<SongDetailed>, ClientError> {
        self.find_songs_with_filter(query, "Eg-KAQwIARAAGAAgACgAMABqChAEEAMQCRAFEAo%3D").await
    }

    pub async fn find_videos(&self, query: &str) -> Result<Vec<VideoDetailed>, ClientError> {
        let data = self.construct_request("search", json!({ "query": query, "params": "Eg-KAQwIABABGAAgACgAMABqChAEEAMQCRAFEAo%3D" }), HashMap::new()).await?;
        Ok(traverse_list(&data, &["musicResponsiveListItemRenderer"]).into_iter().map(VideoParser::parse_search_result).collect())
    }

    pub async fn find_artists(&self, query: &str) -> Result<Vec<ArtistDetailed>, ClientError> {
        let data = self.construct_request("search", json!({ "query": query, "params": "Eg-KAQwIABAAGAAgASgAMABqChAEEAMQCRAFEAo%3D" }), HashMap::new()).await?;
        Ok(traverse_list(&data, &["musicResponsiveListItemRenderer"]).into_iter().map(ArtistParser::parse_search_result).collect())
    }

    pub async fn find_albums(&self, query: &str) -> Result<Vec<AlbumDetailed>, ClientError> {
        let data = self.construct_request("search", json!({ "query": query, "params": "Eg-KAQwIABAAGAEgACgAMABqChAEEAMQCRAFEAo%3D" }), HashMap::new()).await?;
        Ok(traverse_list(&data, &["musicResponsiveListItemRenderer"]).into_iter().map(AlbumParser::parse_search_result).collect())
    }

    pub async fn find_playlists(&self, query: &str) -> Result<Vec<PlaylistDetailed>, ClientError> {
        let data = self.construct_request("search", json!({ "query": query, "params": "Eg-KAQwIABAAGAAgACgBMABqChAEEAMQCRAFEAo%3D" }), HashMap::new()).await?;
        Ok(traverse_list(&data, &["musicResponsiveListItemRenderer"]).into_iter().map(PlaylistParser::parse_search_result).collect())
    }

    async fn find_songs_with_filter(&self, query: &str, params: &str) -> Result<Vec<SongDetailed>, ClientError> {
        let data = self.construct_request("search", json!({ "query": query, "params": params }), HashMap::new()).await?;
        Ok(traverse_list(&data, &["musicResponsiveListItemRenderer"]).into_iter().map(SongParser::parse_search_result).collect())
    }

    pub async fn fetch_song(&self, video_id: &str) -> Result<SongFull, ClientError> {
        validate_video_id(video_id)?;
        let data = self.construct_request("player", json!({ "videoId": video_id }), HashMap::new()).await?;
        let song = SongParser::parse(&data);
        if song.video_id != video_id { return Err(ClientError::InvalidVideoId); }
        Ok(song)
    }

    pub async fn fetch_video(&self, video_id: &str) -> Result<VideoFull, ClientError> {
        validate_video_id(video_id)?;
        let data = self.construct_request("player", json!({ "videoId": video_id }), HashMap::new()).await?;
        let video = VideoParser::parse(&data);
        if video.video_id != video_id { return Err(ClientError::InvalidVideoId); }
        Ok(video)
    }

    pub async fn fetch_watch_queue(&self, video_id: &str) -> Result<Vec<UpNextDetails>, ClientError> {
        validate_video_id(video_id)?;
        let data = self.construct_request("next", json!({ "videoId": video_id, "playlistId": format!("RDAMVM{video_id}"), "isAudioOnly": true }), HashMap::new()).await?;
        let contents = data.pointer("/contents/singleColumnMusicWatchNextResultsRenderer/tabbedRenderer/watchNextTabbedResultsRenderer/tabs/0/tabRenderer/content/musicQueueRenderer/content/playlistPanelRenderer/contents")
            .and_then(Value::as_array)
            .ok_or(ClientError::InvalidResponse)?;
        Ok(contents.iter().skip(1).filter_map(|item| {
            let renderer = item.get("playlistPanelVideoRenderer")?;
            Some(UpNextDetails {
                video_id: renderer.get("videoId")?.as_str()?.to_string(),
                title: traverse_string(renderer, &["title", "runs", "text"]),
                artists: {
                    let artist = Parser::parse_artist_basic(renderer, &["shortBylineText"]);
                    if artist.name.is_empty() {
                        Parser::parse_artist_basic(renderer, &["longBylineText"])
                    } else {
                        artist
                    }
                },
                duration: Parser::parse_duration(Some(&traverse_string(renderer, &["lengthText", "runs", "text"]))),
                thumbnails: Parser::parse_thumbnails(renderer, &["thumbnail", "thumbnails"]),
            })
        }).collect())
    }

    pub async fn fetch_lyrics(&self, video_id: &str) -> Result<Option<Vec<String>>, ClientError> {
        validate_video_id(video_id)?;
        let data = self.construct_request("next", json!({ "videoId": video_id }), HashMap::new()).await?;
        let tabs = traverse_list(&data, &["tabs", "tabRenderer"]);
        let browse_id = tabs.get(1).map(|v| traverse_string(v, &["browseId"])).unwrap_or_default();
        if browse_id.is_empty() { return Ok(None); }
        let lyrics_data = self.construct_request("browse", json!({ "browseId": browse_id }), HashMap::new()).await?;
        let lyrics = traverse_string(&lyrics_data, &["description", "runs", "text"]);
        if lyrics.is_empty() {
            Ok(None)
        } else {
            Ok(Some(lyrics.replace('\r', "").lines().filter(|v| !v.is_empty()).map(ToString::to_string).collect()))
        }
    }

    pub async fn fetch_artist(&self, artist_id: &str) -> Result<ArtistFull, ClientError> {
        let data = self.construct_request("browse", json!({ "browseId": artist_id }), HashMap::new()).await?;
        Ok(ArtistParser::parse(&data, artist_id))
    }

    pub async fn fetch_artist_songs(&self, artist_id: &str) -> Result<Vec<SongDetailed>, ClientError> {
        let artist_data = self.construct_request("browse", json!({ "browseId": artist_id }), HashMap::new()).await?;
        let browse_token = traverse_string(&artist_data, &["musicShelfRenderer", "title", "browseId"]);
        if browse_token.is_empty() { return Ok(vec![]); }

        let artist = crate::types::ArtistBasic {
            artist_id: Some(artist_id.to_string()),
            name: traverse_string(&artist_data, &["header", "title", "text"]),
        };

        let songs_data = self.construct_request("browse", json!({ "browseId": browse_token }), HashMap::new()).await?;
        let mut songs: Vec<SongDetailed> = traverse_list(&songs_data, &["musicResponsiveListItemRenderer"])
            .into_iter()
            .map(|item| SongParser::parse_artist_song(item, artist.clone()))
            .collect();

        let continuation = traverse_string(&songs_data, &["continuation"]);
        if !continuation.is_empty() {
            let mut q = HashMap::new();
            q.insert("continuation".to_string(), continuation);
            let more = self.construct_request("browse", json!({}), q).await?;
            songs.extend(
                traverse_list(&more, &["musicResponsiveListItemRenderer"])
                    .into_iter()
                    .map(|item| SongParser::parse_artist_song(item, artist.clone())),
            );
        }

        Ok(songs)
    }

    pub async fn fetch_artist_albums(&self, artist_id: &str) -> Result<Vec<AlbumDetailed>, ClientError> {
        let artist_data = self.construct_request("browse", json!({ "browseId": artist_id }), HashMap::new()).await?;
        let artist_albums_data = traverse_list(&artist_data, &["musicCarouselShelfRenderer"]).first().copied().ok_or(ClientError::InvalidResponse)?;
        let browse_body = traverse_first(artist_albums_data, &["moreContentButton", "browseEndpoint"]).cloned().unwrap_or_else(|| json!({}));
        let albums_data = self.construct_request("browse", browse_body, HashMap::new()).await?;
        let artist = crate::types::ArtistBasic { artist_id: Some(artist_id.to_string()), name: traverse_string(&albums_data, &["header", "runs", "text"]) };
        Ok(traverse_list(&albums_data, &["musicTwoRowItemRenderer"]).into_iter().map(|item| AlbumParser::parse_artist_album(item, artist.clone())).collect())
    }

    pub async fn fetch_album(&self, album_id: &str) -> Result<AlbumFull, ClientError> {
        let data = self.construct_request("browse", json!({ "browseId": album_id }), HashMap::new()).await?;
        Ok(AlbumParser::parse(&data, album_id))
    }

    pub async fn fetch_playlist(&self, playlist_id: &str) -> Result<PlaylistFull, ClientError> {
        let browse_id = normalize_playlist_id(playlist_id);
        let data = self.construct_request("browse", json!({ "browseId": browse_id }), HashMap::new()).await?;
        Ok(PlaylistParser::parse(&data, &browse_id))
    }

    pub async fn fetch_playlist_tracks(&self, playlist_id: &str) -> Result<Vec<VideoDetailed>, ClientError> {
        let browse_id = normalize_playlist_id(playlist_id);
        let playlist_data = self.construct_request("browse", json!({ "browseId": browse_id }), HashMap::new()).await?;

        let mut videos: Vec<VideoDetailed> = traverse_list(&playlist_data, &["musicPlaylistShelfRenderer", "musicResponsiveListItemRenderer"])
            .into_iter()
            .filter_map(VideoParser::parse_playlist_video)
            .collect();

        let mut continuation = traverse_string(&playlist_data, &["continuation"]);
        while !continuation.is_empty() {
            let mut q = HashMap::new();
            q.insert("continuation".to_string(), continuation.clone());
            let songs_data = self.construct_request("browse", json!({}), q).await?;
            videos.extend(
                traverse_list(&songs_data, &["musicResponsiveListItemRenderer"])
                    .into_iter()
                    .filter_map(VideoParser::parse_playlist_video),
            );
            let next = traverse_string(&songs_data, &["continuation"]);
            if next == continuation { break; }
            continuation = next;
        }

        Ok(videos)
    }

    pub async fn fetch_home(&self) -> Result<Vec<HomeSection>, ClientError> {
        let data = self.construct_request("browse", json!({ "browseId": FE_MUSIC_HOME }), HashMap::new()).await?;

        let mut sections: Vec<HomeSection> = traverse_list(&data, &["sectionListRenderer", "contents"])
            .into_iter()
            .map(Parser::parse_home_section)
            .collect();

        let mut continuation = traverse_string(&data, &["continuation"]);
        while !continuation.is_empty() {
            let mut q = HashMap::new();
            q.insert("continuation".to_string(), continuation.clone());
            let more = self.construct_request("browse", json!({}), q).await?;
            sections.extend(
                traverse_list(&more, &["sectionListContinuation", "contents"])
                    .into_iter()
                    .map(Parser::parse_home_section),
            );
            let next = traverse_string(&more, &["continuation"]);
            if next == continuation { break; }
            continuation = next;
        }

        Ok(sections)
    }
}

fn merge_json(target: &mut Value, source: Value) {
    match (target, source) {
        (Value::Object(a), Value::Object(b)) => {
            for (k, v) in b {
                merge_json(a.entry(k).or_insert(Value::Null), v);
            }
        }
        (target, source) => *target = source,
    }
}

fn validate_video_id(video_id: &str) -> Result<(), ClientError> {
    let valid = video_id.len() == 11 && video_id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
    if valid { Ok(()) } else { Err(ClientError::InvalidVideoId) }
}

fn normalize_playlist_id(playlist_id: &str) -> String {
    if playlist_id.starts_with("PL") { format!("VL{playlist_id}") } else { playlist_id.to_string() }
}
