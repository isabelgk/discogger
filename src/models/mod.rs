pub mod artist;
pub mod label;
pub mod master;
pub mod release;
pub mod search;

use serde::Deserialize;

pub use artist::{Artist, ArtistRelease};
pub use label::{Label, LabelRelease};
pub use master::{MasterRelease, MasterVersion};
pub use release::{Company, Format, Identifier, LabelRef, Video};
pub use release::{Release, Track};
pub use search::{SearchParams, SearchResult, SearchType};

/// Pagination metadata returned by the Discogs API.
#[derive(Debug, Clone, Deserialize)]
pub struct PaginationInfo {
    pub page: u32,
    pub pages: u32,
    #[serde(default)]
    pub per_page: u32,
    #[serde(default)]
    pub items: u32,
}

/// An image associated with a resource.
#[derive(Debug, Clone, Deserialize)]
pub struct Image {
    #[serde(rename = "type")]
    pub image_type: Option<String>,
    pub uri: Option<String>,
    pub uri150: Option<String>,
    pub resource_url: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

/// A summary reference to an artist (used in releases, labels, etc.).
#[derive(Debug, Clone, Deserialize)]
pub struct ArtistSummary {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub resource_url: Option<String>,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub join: Option<String>,
    #[serde(default)]
    pub anv: Option<String>,
    #[serde(default)]
    pub tracks: Option<String>,
}
