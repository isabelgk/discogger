use serde::Deserialize;

use super::{ArtistSummary, Image};

/// A full artist resource from the Discogs API.
#[derive(Debug, Clone, Deserialize)]
pub struct Artist {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub resource_url: Option<String>,
    #[serde(default)]
    pub uri: Option<String>,
    #[serde(default)]
    pub releases_url: Option<String>,
    #[serde(default)]
    pub profile: Option<String>,
    #[serde(default)]
    pub urls: Vec<String>,
    #[serde(default)]
    pub namevariations: Vec<String>,
    #[serde(default)]
    pub aliases: Vec<ArtistSummary>,
    #[serde(default)]
    pub members: Vec<ArtistSummary>,
    #[serde(default)]
    pub groups: Vec<ArtistSummary>,
    #[serde(default)]
    pub images: Vec<Image>,
    #[serde(default)]
    pub data_quality: Option<String>,
    #[serde(default)]
    pub realname: Option<String>,
}

/// An artist's release as returned by the artist releases endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct ArtistRelease {
    pub id: u64,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub year: Option<u32>,
    #[serde(default)]
    pub resource_url: Option<String>,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default, rename = "type")]
    pub release_type: Option<String>,
    #[serde(default)]
    pub main_release: Option<u64>,
    #[serde(default)]
    pub artist: Option<String>,
    #[serde(default)]
    pub format: Option<String>,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub thumb: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_artist() {
        let json = r#"{
            "id": 108713,
            "name": "Aphex Twin",
            "resource_url": "https://api.discogs.com/artists/108713",
            "profile": "British electronic musician.",
            "urls": ["https://aphextwin.warp.net/"],
            "namevariations": ["AFX", "The Aphex Twin"],
            "images": [],
            "members": [],
            "aliases": []
        }"#;
        let artist: Artist = serde_json::from_str(json).unwrap();
        assert_eq!(artist.id, 108713);
        assert_eq!(artist.name, "Aphex Twin");
        assert_eq!(
            artist.profile.as_deref(),
            Some("British electronic musician.")
        );
    }

    #[test]
    fn test_deserialize_artist_release() {
        let json = r#"{
            "id": 20209,
            "title": "Selected Ambient Works 85-92",
            "year": 1992,
            "type": "master",
            "role": "Main"
        }"#;
        let release: ArtistRelease = serde_json::from_str(json).unwrap();
        assert_eq!(release.id, 20209);
        assert_eq!(
            release.title.as_deref(),
            Some("Selected Ambient Works 85-92")
        );
        assert_eq!(release.year, Some(1992));
        assert_eq!(release.release_type.as_deref(), Some("master"));
    }
}
