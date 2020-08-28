use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RaiPlayVideo {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "type")]
    pub rai_play_video_type: String,

    #[serde(rename = "date_published")]
    pub date_published: String,

    #[serde(rename = "time_published")]
    pub time_published: String,

    #[serde(rename = "path_id")]
    pub path_id: String,

    #[serde(rename = "weblink")]
    pub weblink: String,

    #[serde(rename = "info_url")]
    pub info_url: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "subtitle")]
    pub subtitle: String,

    #[serde(rename = "label")]
    pub label: String,

    #[serde(rename = "season")]
    pub season: String,

    #[serde(rename = "episode")]
    pub episode: String,

    #[serde(rename = "episode_title")]
    pub episode_title: String,

    #[serde(rename = "direction")]
    pub direction: String,

    #[serde(rename = "actors")]
    pub actors: String,

    #[serde(rename = "rating")]
    pub rating: String,

    #[serde(rename = "author")]
    pub author: String,

    #[serde(rename = "editor")]
    pub editor: String,

    #[serde(rename = "channel")]
    pub channel: String,

    #[serde(rename = "genres")]
    pub genres: Vec<Option<serde_json::Value>>,

    #[serde(rename = "subgenres")]
    pub subgenres: Vec<Option<serde_json::Value>>,

    #[serde(rename = "collections")]
    pub collections: Vec<Option<serde_json::Value>>,

    #[serde(rename = "caption")]
    pub caption: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "form")]
    pub form: String,

    #[serde(rename = "video")]
    pub video: Video,

    #[serde(rename = "images")]
    pub images: Images,

    #[serde(rename = "details")]
    pub details: Vec<Detail>,

    #[serde(rename = "login_required")]
    pub login_required: bool,

    #[serde(rename = "parent_page")]
    pub parent_page: String,

    #[serde(rename = "related")]
    pub related: String,

    #[serde(rename = "availabilities")]
    pub availabilities: Option<serde_json::Value>,

    #[serde(rename = "adv")]
    pub adv: bool,

    #[serde(rename = "dfp")]
    pub dfp: GeoprotectionClass,

    #[serde(rename = "rights_management")]
    pub rights_management: RightsManagement,

    #[serde(rename = "program_info")]
    pub program_info: ProgramInfo,

    #[serde(rename = "is_live")]
    pub is_live: bool,

    #[serde(rename = "track_info")]
    pub track_info: TrackInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Detail {
    #[serde(rename = "type")]
    pub detail_type: String,

    #[serde(rename = "key")]
    pub key: String,

    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeoprotectionClass {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Images {
    #[serde(rename = "landscape")]
    pub landscape: String,

    #[serde(rename = "portrait")]
    pub portrait: String,

    #[serde(rename = "square")]
    pub square: String,

    #[serde(rename = "landscape43")]
    pub landscape43: String,

    #[serde(rename = "portrait43")]
    pub portrait43: String,

    #[serde(rename = "portrait_logo")]
    pub portrait_logo: String,

    #[serde(rename = "landscape_logo")]
    pub landscape_logo: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgramInfo {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "channel")]
    pub channel: String,

    #[serde(rename = "editor")]
    pub editor: String,

    #[serde(rename = "social")]
    pub social: Social,

    #[serde(rename = "layout")]
    pub layout: String,

    #[serde(rename = "onair_date")]
    pub onair_date: String,

    #[serde(rename = "subtitle")]
    pub subtitle: String,

    #[serde(rename = "website")]
    pub website: String,

    #[serde(rename = "year")]
    pub year: String,

    #[serde(rename = "direction")]
    pub direction: String,

    #[serde(rename = "production")]
    pub production: String,

    #[serde(rename = "country")]
    pub country: String,

    #[serde(rename = "actors")]
    pub actors: String,

    #[serde(rename = "presenter")]
    pub presenter: String,

    #[serde(rename = "seasons_number")]
    pub seasons_number: String,

    #[serde(rename = "rating")]
    pub rating: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "vanity")]
    pub vanity: String,

    #[serde(rename = "label")]
    pub label: String,

    #[serde(rename = "weblink")]
    pub weblink: String,

    #[serde(rename = "info_url")]
    pub info_url: String,

    #[serde(rename = "path_id")]
    pub path_id: String,

    #[serde(rename = "typology")]
    pub typology: String,

    #[serde(rename = "typologies")]
    pub typologies: Vec<Genre>,

    #[serde(rename = "genres")]
    pub genres: Vec<Genre>,

    #[serde(rename = "subgenres")]
    pub subgenres: Vec<Genre>,

    #[serde(rename = "related")]
    pub related: String,

    #[serde(rename = "az")]
    pub az: bool,

    #[serde(rename = "play_service_inverted")]
    pub play_service_inverted: bool,

    #[serde(rename = "details")]
    pub details: Vec<Detail>,

    #[serde(rename = "images")]
    pub images: Images,

    #[serde(rename = "program_category")]
    pub program_category: ProgramCategory,

    #[serde(rename = "rights_management")]
    pub rights_management: RightsManagement,

    #[serde(rename = "fmt")]
    pub fmt: String,

    #[serde(rename = "keyw")]
    pub keyw: String,

    #[serde(rename = "banner300x250")]
    pub banner300_x250: String,

    #[serde(rename = "bannerUnicoLoc")]
    pub banner_unico_loc: String,

    #[serde(rename = "dfp")]
    pub dfp: ProgramInfoDfp,

    #[serde(rename = "adv")]
    pub adv: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgramInfoDfp {
    #[serde(rename = "escaped_name")]
    pub escaped_name: String,

    #[serde(rename = "label")]
    pub label: String,

    #[serde(rename = "escaped_genres")]
    pub escaped_genres: Vec<ProgramCategory>,

    #[serde(rename = "escaped_typology")]
    pub escaped_typology: Vec<ProgramCategory>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgramCategory {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Genre {
    #[serde(rename = "principal")]
    pub principal: Option<bool>,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "nome")]
    pub nome: String,

    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RightsManagement {
    #[serde(rename = "rights")]
    pub rights: Rights,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rights {
    #[serde(rename = "offline")]
    pub offline: GeoprotectionClass,

    #[serde(rename = "geoprotection")]
    pub geoprotection: GeoprotectionClass,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Social {
    #[serde(rename = "sms")]
    pub sms: String,

    #[serde(rename = "phone_number")]
    pub phone_number: String,

    #[serde(rename = "whatsapp")]
    pub whatsapp: String,

    #[serde(rename = "email")]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackInfo {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "domain")]
    pub domain: String,

    #[serde(rename = "platform")]
    pub platform: String,

    #[serde(rename = "media_type")]
    pub media_type: String,

    #[serde(rename = "page_type")]
    pub page_type: String,

    #[serde(rename = "editor")]
    pub editor: String,

    #[serde(rename = "year")]
    pub year: String,

    #[serde(rename = "section")]
    pub section: String,

    #[serde(rename = "sub_section")]
    pub sub_section: String,

    #[serde(rename = "content")]
    pub content: String,

    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "channel")]
    pub channel: String,

    #[serde(rename = "date")]
    pub date: String,

    #[serde(rename = "typology")]
    pub typology: String,

    #[serde(rename = "genres")]
    pub genres: Option<serde_json::Value>,

    #[serde(rename = "sub_genres")]
    pub sub_genres: Option<serde_json::Value>,

    #[serde(rename = "program_title")]
    pub program_title: String,

    #[serde(rename = "program_typology")]
    pub program_typology: String,

    #[serde(rename = "program_genres")]
    pub program_genres: Vec<String>,

    #[serde(rename = "program_sub_genres")]
    pub program_sub_genres: Vec<String>,

    #[serde(rename = "edition")]
    pub edition: String,

    #[serde(rename = "season")]
    pub season: String,

    #[serde(rename = "episode_number")]
    pub episode_number: String,

    #[serde(rename = "episode_title")]
    pub episode_title: String,

    #[serde(rename = "form")]
    pub form: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Video {
    #[serde(rename = "content_url")]
    pub content_url: String,

    #[serde(rename = "duration")]
    pub duration: String,

    #[serde(rename = "highlights")]
    pub highlights: String,

    #[serde(rename = "subtitles")]
    pub subtitles: String,

    #[serde(rename = "subtitlesArray")]
    pub subtitles_array: Vec<Option<serde_json::Value>>,
}
