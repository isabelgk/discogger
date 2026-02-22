//! Blocking (synchronous) API.
//!
//! Enabled with the `blocking` feature flag. Wraps the async [`DiscogsClient`](crate::DiscogsClient)
//! and drives it on a single-threaded tokio runtime.
//!
//! # Example
//!
//! ```no_run
//! use discogger::blocking::DiscogsClient;
//!
//! let client = DiscogsClient::builder()
//!     .user_agent("MyApp/1.0 +https://example.com")
//!     .personal_token("my-token")
//!     .build()
//!     .unwrap();
//!
//! let artist = client.artist(45).unwrap();
//! println!("{}", artist.name);
//! ```

use bytes::Bytes;
use tokio::runtime::Builder;

use crate::client::CoverArt;
use crate::error::Result;
use crate::models::artist::{Artist, ArtistRelease};
use crate::models::label::{Label, LabelRelease};
use crate::models::master::{MasterRelease, MasterVersion};
use crate::models::release::Release;
use crate::models::search::{SearchParams, SearchResult};
use crate::pagination::{Paginated, PaginationParams};
use crate::DiscogsError;

/// Blocking (synchronous) Discogs client.
pub struct DiscogsClient {
    inner: crate::DiscogsClient,
    runtime: tokio::runtime::Runtime,
}

/// Builder for a blocking [`DiscogsClient`].
pub struct ClientBuilder(crate::ClientBuilder);

impl DiscogsClient {
    /// Create a new blocking client builder.
    pub fn builder() -> ClientBuilder {
        ClientBuilder(crate::DiscogsClient::builder())
    }
}

impl ClientBuilder {
    /// Set the User-Agent header (required by Discogs API).
    pub fn user_agent(self, ua: impl Into<String>) -> Self {
        Self(self.0.user_agent(ua))
    }

    /// Override the base URL. For testing only.
    #[doc(hidden)]
    pub fn base_url(self, url: impl Into<String>) -> Self {
        Self(self.0.base_url(url))
    }

    /// Authenticate with a personal access token.
    pub fn personal_token(self, token: impl Into<String>) -> Self {
        Self(self.0.personal_token(token))
    }

    /// Authenticate with OAuth 1.0a credentials.
    pub fn oauth(
        self,
        consumer_key: impl Into<String>,
        consumer_secret: impl Into<String>,
        token: impl Into<String>,
        token_secret: impl Into<String>,
    ) -> Self {
        Self(self.0.oauth(consumer_key, consumer_secret, token, token_secret))
    }

    /// Build the blocking client.
    pub fn build(self) -> Result<DiscogsClient> {
        let inner = self.0.build()?;
        let runtime = Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| DiscogsError::Configuration(e.to_string()))?;
        Ok(DiscogsClient { inner, runtime })
    }
}

impl DiscogsClient {
    /// Get an artist by ID.
    pub fn artist(&self, id: u64) -> Result<Artist> {
        self.runtime.block_on(self.inner.artist(id))
    }

    /// Get an artist's releases.
    pub fn artist_releases(
        &self,
        id: u64,
        pagination: &PaginationParams,
    ) -> Result<Paginated<ArtistRelease>> {
        self.runtime.block_on(self.inner.artist_releases(id, pagination))
    }

    /// Get a release by ID.
    pub fn release(&self, id: u64) -> Result<Release> {
        self.runtime.block_on(self.inner.release(id))
    }

    /// Get a label by ID.
    pub fn label(&self, id: u64) -> Result<Label> {
        self.runtime.block_on(self.inner.label(id))
    }

    /// Get a label's releases.
    pub fn label_releases(
        &self,
        id: u64,
        pagination: &PaginationParams,
    ) -> Result<Paginated<LabelRelease>> {
        self.runtime.block_on(self.inner.label_releases(id, pagination))
    }

    /// Get a master release by ID.
    pub fn master(&self, id: u64) -> Result<MasterRelease> {
        self.runtime.block_on(self.inner.master(id))
    }

    /// Get versions of a master release.
    pub fn master_versions(
        &self,
        id: u64,
        pagination: &PaginationParams,
    ) -> Result<Paginated<MasterVersion>> {
        self.runtime.block_on(self.inner.master_versions(id, pagination))
    }

    /// Search the Discogs database.
    pub fn search(
        &self,
        params: &SearchParams,
        pagination: &PaginationParams,
    ) -> Result<Paginated<SearchResult>> {
        self.runtime.block_on(self.inner.search(params, pagination))
    }

    /// Download an image from a Discogs image URL, returning the raw bytes.
    pub fn download_image(&self, url: &str) -> Result<Bytes> {
        self.runtime.block_on(self.inner.download_image(url))
    }

    /// Fetch a release and download its primary cover image.
    pub fn release_cover_art(&self, id: u64) -> Result<Option<CoverArt>> {
        self.runtime.block_on(self.inner.release_cover_art(id))
    }
}
