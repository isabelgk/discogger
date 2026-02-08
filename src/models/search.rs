use serde::Deserialize;
use std::fmt;

/// The type of resource to search for.
#[derive(Debug, Clone)]
pub enum SearchType {
    Release,
    Master,
    Artist,
    Label,
}

impl fmt::Display for SearchType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SearchType::Release => write!(f, "release"),
            SearchType::Master => write!(f, "master"),
            SearchType::Artist => write!(f, "artist"),
            SearchType::Label => write!(f, "label"),
        }
    }
}

/// Parameters for the search endpoint.
#[derive(Debug, Clone, Default)]
pub struct SearchParams {
    pub query: Option<String>,
    pub search_type: Option<SearchType>,
    pub title: Option<String>,
    pub release_title: Option<String>,
    pub artist: Option<String>,
    pub label: Option<String>,
    pub genre: Option<String>,
    pub style: Option<String>,
    pub country: Option<String>,
    pub year: Option<String>,
    pub format: Option<String>,
    pub catno: Option<String>,
    pub barcode: Option<String>,
}

impl SearchParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn query(mut self, q: impl Into<String>) -> Self {
        self.query = Some(q.into());
        self
    }

    pub fn search_type(mut self, t: SearchType) -> Self {
        self.search_type = Some(t);
        self
    }

    pub fn artist(mut self, a: impl Into<String>) -> Self {
        self.artist = Some(a.into());
        self
    }

    pub fn title(mut self, t: impl Into<String>) -> Self {
        self.title = Some(t.into());
        self
    }

    pub fn label(mut self, l: impl Into<String>) -> Self {
        self.label = Some(l.into());
        self
    }

    pub fn genre(mut self, g: impl Into<String>) -> Self {
        self.genre = Some(g.into());
        self
    }

    pub fn style(mut self, s: impl Into<String>) -> Self {
        self.style = Some(s.into());
        self
    }

    pub fn country(mut self, c: impl Into<String>) -> Self {
        self.country = Some(c.into());
        self
    }

    pub fn year(mut self, y: impl Into<String>) -> Self {
        self.year = Some(y.into());
        self
    }

    pub fn format(mut self, f: impl Into<String>) -> Self {
        self.format = Some(f.into());
        self
    }

    pub fn catno(mut self, c: impl Into<String>) -> Self {
        self.catno = Some(c.into());
        self
    }

    pub fn barcode(mut self, b: impl Into<String>) -> Self {
        self.barcode = Some(b.into());
        self
    }

    pub(crate) fn as_query_pairs(&self) -> Vec<(&str, String)> {
        let mut pairs = Vec::new();
        if let Some(ref q) = self.query {
            pairs.push(("q", q.clone()));
        }
        if let Some(ref t) = self.search_type {
            pairs.push(("type", t.to_string()));
        }
        if let Some(ref v) = self.title {
            pairs.push(("title", v.clone()));
        }
        if let Some(ref v) = self.release_title {
            pairs.push(("release_title", v.clone()));
        }
        if let Some(ref v) = self.artist {
            pairs.push(("artist", v.clone()));
        }
        if let Some(ref v) = self.label {
            pairs.push(("label", v.clone()));
        }
        if let Some(ref v) = self.genre {
            pairs.push(("genre", v.clone()));
        }
        if let Some(ref v) = self.style {
            pairs.push(("style", v.clone()));
        }
        if let Some(ref v) = self.country {
            pairs.push(("country", v.clone()));
        }
        if let Some(ref v) = self.year {
            pairs.push(("year", v.clone()));
        }
        if let Some(ref v) = self.format {
            pairs.push(("format", v.clone()));
        }
        if let Some(ref v) = self.catno {
            pairs.push(("catno", v.clone()));
        }
        if let Some(ref v) = self.barcode {
            pairs.push(("barcode", v.clone()));
        }
        pairs
    }
}

/// A single search result from the Discogs API.
#[derive(Debug, Clone, Deserialize)]
pub struct SearchResult {
    pub id: u64,
    #[serde(default, rename = "type")]
    pub result_type: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub resource_url: Option<String>,
    #[serde(default)]
    pub uri: Option<String>,
    #[serde(default)]
    pub thumb: Option<String>,
    #[serde(default)]
    pub cover_image: Option<String>,
    #[serde(default)]
    pub master_id: Option<u64>,
    #[serde(default)]
    pub master_url: Option<String>,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub year: Option<String>,
    #[serde(default)]
    pub format: Vec<String>,
    #[serde(default)]
    pub label: Vec<String>,
    #[serde(default)]
    pub genre: Vec<String>,
    #[serde(default)]
    pub style: Vec<String>,
    #[serde(default)]
    pub catno: Option<String>,
    #[serde(default)]
    pub barcode: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_search_result() {
        let json = r#"{
            "id": 108713,
            "type": "artist",
            "title": "Aphex Twin",
            "resource_url": "https://api.discogs.com/artists/108713",
            "thumb": "https://img.discogs.com/thumb.jpg",
            "cover_image": "https://img.discogs.com/cover.jpg"
        }"#;
        let result: SearchResult = serde_json::from_str(json).unwrap();
        assert_eq!(result.id, 108713);
        assert_eq!(result.result_type.as_deref(), Some("artist"));
        assert_eq!(result.title.as_deref(), Some("Aphex Twin"));
    }

    #[test]
    fn test_search_params_builder() {
        let params = SearchParams::new()
            .query("aphex twin")
            .search_type(SearchType::Artist)
            .country("UK");
        let pairs = params.as_query_pairs();
        assert!(pairs.iter().any(|(k, v)| *k == "q" && v == "aphex twin"));
        assert!(pairs.iter().any(|(k, v)| *k == "type" && v == "artist"));
        assert!(pairs.iter().any(|(k, v)| *k == "country" && v == "UK"));
    }
}
