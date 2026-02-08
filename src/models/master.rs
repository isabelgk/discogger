use serde::Deserialize;

use super::{ArtistSummary, Image};
use crate::models::release::Track;

/// A master release from the Discogs API.
#[derive(Debug, Clone, Deserialize)]
pub struct MasterRelease {
    pub id: u64,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub year: Option<u32>,
    #[serde(default)]
    pub resource_url: Option<String>,
    #[serde(default)]
    pub uri: Option<String>,
    #[serde(default)]
    pub main_release: Option<u64>,
    #[serde(default)]
    pub main_release_url: Option<String>,
    #[serde(default)]
    pub versions_url: Option<String>,
    #[serde(default)]
    pub most_recent_release: Option<u64>,
    #[serde(default)]
    pub most_recent_release_url: Option<String>,
    #[serde(default)]
    pub artists: Vec<ArtistSummary>,
    #[serde(default)]
    pub genres: Vec<String>,
    #[serde(default)]
    pub styles: Vec<String>,
    #[serde(default)]
    pub tracklist: Vec<Track>,
    #[serde(default)]
    pub images: Vec<Image>,
    #[serde(default)]
    pub data_quality: Option<String>,
    #[serde(default)]
    pub num_for_sale: Option<u32>,
    #[serde(default)]
    pub lowest_price: Option<f64>,
}

/// A version of a master release.
#[derive(Debug, Clone, Deserialize)]
pub struct MasterVersion {
    pub id: u64,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub year: Option<u32>,
    #[serde(default)]
    pub resource_url: Option<String>,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub format: Option<String>,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub catno: Option<String>,
    #[serde(default)]
    pub released: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub thumb: Option<String>,
    #[serde(default)]
    pub major_formats: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_master() {
        let json = r#"{
            "id": 4148,
            "title": "Selected Ambient Works 85-92",
            "year": 1992,
            "main_release": 67896,
            "artists": [{"id": 108713, "name": "Aphex Twin"}],
            "genres": ["Electronic"],
            "styles": ["Ambient"],
            "tracklist": [],
            "images": []
        }"#;
        let master: MasterRelease = serde_json::from_str(json).unwrap();
        assert_eq!(master.id, 4148);
        assert_eq!(
            master.title.as_deref(),
            Some("Selected Ambient Works 85-92")
        );
        assert_eq!(master.main_release, Some(67896));
    }

    #[test]
    fn test_deserialize_master_version() {
        let json = r#"{
            "id": 67896,
            "title": "Selected Ambient Works 85-92",
            "year": 1992,
            "country": "UK",
            "format": "CD, Album",
            "label": "Apollo",
            "catno": "AMB LP 3922"
        }"#;
        let version: MasterVersion = serde_json::from_str(json).unwrap();
        assert_eq!(version.id, 67896);
        assert_eq!(version.country.as_deref(), Some("UK"));
    }
}
