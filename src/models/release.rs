use serde::Deserialize;

use super::{ArtistSummary, Image};

/// A full release resource from the Discogs API.
#[derive(Debug, Clone, Deserialize)]
pub struct Release {
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
    pub artists: Vec<ArtistSummary>,
    #[serde(default)]
    pub extraartists: Vec<ArtistSummary>,
    #[serde(default)]
    pub labels: Vec<LabelRef>,
    #[serde(default)]
    pub formats: Vec<Format>,
    #[serde(default)]
    pub tracklist: Vec<Track>,
    #[serde(default)]
    pub genres: Vec<String>,
    #[serde(default)]
    pub styles: Vec<String>,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub released: Option<String>,
    #[serde(default)]
    pub released_formatted: Option<String>,
    #[serde(default)]
    pub master_id: Option<u64>,
    #[serde(default)]
    pub master_url: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub data_quality: Option<String>,
    #[serde(default)]
    pub images: Vec<Image>,
    #[serde(default)]
    pub thumb: Option<String>,
    #[serde(default)]
    pub videos: Vec<Video>,
    #[serde(default)]
    pub identifiers: Vec<Identifier>,
    #[serde(default)]
    pub companies: Vec<Company>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub num_for_sale: Option<u32>,
    #[serde(default)]
    pub lowest_price: Option<f64>,
}

/// A track in a release's tracklist.
#[derive(Debug, Clone, Deserialize)]
pub struct Track {
    #[serde(default)]
    pub position: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub duration: Option<String>,
    #[serde(default, rename = "type_")]
    pub track_type: Option<String>,
    #[serde(default)]
    pub artists: Vec<ArtistSummary>,
    #[serde(default)]
    pub extraartists: Vec<ArtistSummary>,
}

/// Format information for a release.
#[derive(Debug, Clone, Deserialize)]
pub struct Format {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub qty: Option<String>,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub descriptions: Vec<String>,
}

/// A reference to a label within a release.
#[derive(Debug, Clone, Deserialize)]
pub struct LabelRef {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub catno: Option<String>,
    #[serde(default)]
    pub resource_url: Option<String>,
    #[serde(default)]
    pub entity_type: Option<String>,
    #[serde(default)]
    pub entity_type_name: Option<String>,
}

/// A video associated with a release.
#[derive(Debug, Clone, Deserialize)]
pub struct Video {
    #[serde(default)]
    pub uri: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub duration: Option<u32>,
    #[serde(default)]
    pub embed: Option<bool>,
}

/// An identifier (barcode, matrix, etc.) for a release.
#[derive(Debug, Clone, Deserialize)]
pub struct Identifier {
    #[serde(default, rename = "type")]
    pub identifier_type: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

/// A company associated with a release.
#[derive(Debug, Clone, Deserialize)]
pub struct Company {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub catno: Option<String>,
    #[serde(default)]
    pub resource_url: Option<String>,
    #[serde(default)]
    pub entity_type: Option<String>,
    #[serde(default)]
    pub entity_type_name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_release() {
        let json = r#"{
            "id": 249504,
            "title": "Never Gonna Give You Up",
            "year": 1987,
            "artists": [{"id": 72872, "name": "Rick Astley"}],
            "labels": [{"id": 895, "name": "RCA", "catno": "PB 49801"}],
            "formats": [{"name": "Vinyl", "qty": "1", "descriptions": ["7\"", "Single"]}],
            "tracklist": [
                {"position": "A", "title": "Never Gonna Give You Up", "duration": "3:32"},
                {"position": "B", "title": "Together Forever", "duration": "3:24"}
            ],
            "genres": ["Electronic", "Pop"],
            "styles": ["Euro House", "Synth-pop"],
            "country": "UK"
        }"#;
        let release: Release = serde_json::from_str(json).unwrap();
        assert_eq!(release.id, 249504);
        assert_eq!(release.title.as_deref(), Some("Never Gonna Give You Up"));
        assert_eq!(release.tracklist.len(), 2);
        assert_eq!(
            release.tracklist[0].title.as_deref(),
            Some("Never Gonna Give You Up")
        );
        assert_eq!(release.genres, vec!["Electronic", "Pop"]);
    }

    #[test]
    fn test_deserialize_track() {
        let json = r#"{
            "position": "A1",
            "title": "Test Track",
            "duration": "5:30"
        }"#;
        let track: Track = serde_json::from_str(json).unwrap();
        assert_eq!(track.position.as_deref(), Some("A1"));
        assert_eq!(track.title.as_deref(), Some("Test Track"));
        assert_eq!(track.duration.as_deref(), Some("5:30"));
    }
}
