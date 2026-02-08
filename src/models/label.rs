use serde::Deserialize;

use super::Image;

/// A full label resource from the Discogs API.
#[derive(Debug, Clone, Deserialize)]
pub struct Label {
    pub id: u64,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub resource_url: Option<String>,
    #[serde(default)]
    pub uri: Option<String>,
    #[serde(default)]
    pub releases_url: Option<String>,
    #[serde(default)]
    pub profile: Option<String>,
    #[serde(default)]
    pub contact_info: Option<String>,
    #[serde(default)]
    pub data_quality: Option<String>,
    #[serde(default)]
    pub urls: Vec<String>,
    #[serde(default)]
    pub images: Vec<Image>,
    #[serde(default)]
    pub sublabels: Vec<LabelSummary>,
    #[serde(default)]
    pub parent_label: Option<LabelSummary>,
}

/// A summary reference to a label.
#[derive(Debug, Clone, Deserialize)]
pub struct LabelSummary {
    pub id: u64,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub resource_url: Option<String>,
}

/// A release associated with a label.
#[derive(Debug, Clone, Deserialize)]
pub struct LabelRelease {
    pub id: u64,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub year: Option<u32>,
    #[serde(default)]
    pub resource_url: Option<String>,
    #[serde(default)]
    pub artist: Option<String>,
    #[serde(default)]
    pub catno: Option<String>,
    #[serde(default)]
    pub format: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub thumb: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_label() {
        let json = r#"{
            "id": 1,
            "name": "Warp Records",
            "profile": "UK electronic music label",
            "urls": ["https://warp.net"],
            "sublabels": [{"id": 2, "name": "Lex Records"}],
            "images": []
        }"#;
        let label: Label = serde_json::from_str(json).unwrap();
        assert_eq!(label.id, 1);
        assert_eq!(label.name.as_deref(), Some("Warp Records"));
        assert_eq!(label.sublabels.len(), 1);
    }

    #[test]
    fn test_deserialize_label_release() {
        let json = r#"{
            "id": 12345,
            "title": "Test Release",
            "year": 2020,
            "artist": "Test Artist",
            "catno": "WARP001"
        }"#;
        let lr: LabelRelease = serde_json::from_str(json).unwrap();
        assert_eq!(lr.id, 12345);
        assert_eq!(lr.catno.as_deref(), Some("WARP001"));
    }
}
