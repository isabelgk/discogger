use std::sync::Arc;

use bytes::Bytes;
use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::auth::Auth;
use crate::error::{DiscogsError, Result};
use crate::models::artist::{Artist, ArtistRelease};
use crate::models::label::{Label, LabelRelease};
use crate::models::master::{MasterRelease, MasterVersion};
use crate::models::release::Release;
use crate::models::search::{SearchParams, SearchResult};
use crate::models::Image;
use crate::pagination::{Paginated, PaginatedResponse, PaginationParams};
use crate::rate_limit::RateLimiter;

const BASE_URL: &str = "https://api.discogs.com";

struct Inner {
    http: Client,
    auth: Option<Auth>,
    rate_limiter: RateLimiter,
    base_url: String,
}

/// A client for interacting with the Discogs API.
///
/// Cheaply cloneable — wraps an `Arc` internally.
#[derive(Clone)]
pub struct DiscogsClient {
    inner: Arc<Inner>,
}

/// Builder for creating a `DiscogsClient`.
pub struct ClientBuilder {
    user_agent: Option<String>,
    auth: Option<Auth>,
    base_url: String,
}

impl ClientBuilder {
    fn new() -> Self {
        Self {
            user_agent: None,
            auth: None,
            base_url: BASE_URL.to_string(),
        }
    }

    /// Override the base URL. For testing only.
    #[doc(hidden)]
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Set the User-Agent header (required by Discogs API).
    pub fn user_agent(mut self, ua: impl Into<String>) -> Self {
        self.user_agent = Some(ua.into());
        self
    }

    /// Authenticate with a personal access token.
    pub fn personal_token(mut self, token: impl Into<String>) -> Self {
        self.auth = Some(Auth::PersonalToken(token.into()));
        self
    }

    /// Authenticate with OAuth 1.0a credentials.
    pub fn oauth(
        mut self,
        consumer_key: impl Into<String>,
        consumer_secret: impl Into<String>,
        token: impl Into<String>,
        token_secret: impl Into<String>,
    ) -> Self {
        self.auth = Some(Auth::OAuth {
            consumer_key: consumer_key.into(),
            consumer_secret: consumer_secret.into(),
            token: token.into(),
            token_secret: token_secret.into(),
        });
        self
    }

    /// Build the client.
    pub fn build(self) -> Result<DiscogsClient> {
        let user_agent = self.user_agent.ok_or_else(|| {
            DiscogsError::Configuration("User-Agent is required by the Discogs API".into())
        })?;

        if user_agent.is_empty() {
            return Err(DiscogsError::Configuration(
                "User-Agent must not be empty".into(),
            ));
        }

        let max_per_minute = if self.auth.is_some() { 60 } else { 25 };

        let http = Client::builder()
            .user_agent(&user_agent)
            .build()
            .map_err(DiscogsError::Http)?;

        Ok(DiscogsClient {
            inner: Arc::new(Inner {
                http,
                auth: self.auth,
                rate_limiter: RateLimiter::new(max_per_minute),
                base_url: self.base_url,
            }),
        })
    }
}

impl DiscogsClient {
    /// Create a new client builder.
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    /// Internal GET helper that handles auth, rate limiting, and error responses.
    async fn get<T: DeserializeOwned>(&self, path: &str, query: &[(&str, String)]) -> Result<T> {
        self.inner.rate_limiter.acquire().await;

        let url = format!("{}{path}", self.inner.base_url);

        let mut builder = self.inner.http.get(&url);

        if !query.is_empty() {
            builder = builder.query(query);
        }

        // Apply authentication
        if let Some(ref auth) = self.inner.auth {
            // For OAuth, we need the full URL with query params for signing.
            // Build the full URL first.
            let full_url = if !query.is_empty() {
                let params: String = query
                    .iter()
                    .map(|(k, v)| format!("{k}={v}"))
                    .collect::<Vec<_>>()
                    .join("&");
                format!("{url}?{params}")
            } else {
                url.clone()
            };
            builder = auth.apply(builder, "GET", &full_url);
        }

        let response = builder.send().await?;

        // Sync rate limiter with server headers
        if let (Some(used), Some(limit)) = (
            response
                .headers()
                .get("X-Discogs-Ratelimit-Used")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u32>().ok()),
            response
                .headers()
                .get("X-Discogs-Ratelimit")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u32>().ok()),
        ) {
            self.inner.rate_limiter.sync_from_headers(used, limit).await;
        }

        let status = response.status();

        if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            return Err(DiscogsError::RateLimited);
        }

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(DiscogsError::Api {
                status: status.as_u16(),
                body,
            });
        }

        let body = response.text().await?;
        let parsed: T = serde_json::from_str(&body)?;
        Ok(parsed)
    }

    /// Helper for paginated GET requests.
    async fn get_paginated<T: DeserializeOwned>(
        &self,
        path: &str,
        pagination: &PaginationParams,
        extra_query: &[(&str, String)],
    ) -> Result<Paginated<T>> {
        let mut query = pagination.as_query_pairs();
        query.extend_from_slice(extra_query);

        let response: PaginatedResponse<T> = self.get(path, &query).await?;
        Ok(Paginated::new(
            response.data.into_vec(),
            response.pagination,
        ))
    }

    /// Get an artist by ID.
    pub async fn artist(&self, id: u64) -> Result<Artist> {
        self.get(&format!("/artists/{id}"), &[]).await
    }

    /// Get an artist's releases.
    pub async fn artist_releases(
        &self,
        id: u64,
        pagination: &PaginationParams,
    ) -> Result<Paginated<ArtistRelease>> {
        self.get_paginated(&format!("/artists/{id}/releases"), pagination, &[])
            .await
    }

    /// Get a release by ID.
    pub async fn release(&self, id: u64) -> Result<Release> {
        self.get(&format!("/releases/{id}"), &[]).await
    }

    /// Get a label by ID.
    pub async fn label(&self, id: u64) -> Result<Label> {
        self.get(&format!("/labels/{id}"), &[]).await
    }

    /// Get a label's releases.
    pub async fn label_releases(
        &self,
        id: u64,
        pagination: &PaginationParams,
    ) -> Result<Paginated<LabelRelease>> {
        self.get_paginated(&format!("/labels/{id}/releases"), pagination, &[])
            .await
    }

    /// Get a master release by ID.
    pub async fn master(&self, id: u64) -> Result<MasterRelease> {
        self.get(&format!("/masters/{id}"), &[]).await
    }

    /// Get versions of a master release.
    pub async fn master_versions(
        &self,
        id: u64,
        pagination: &PaginationParams,
    ) -> Result<Paginated<MasterVersion>> {
        self.get_paginated(&format!("/masters/{id}/versions"), pagination, &[])
            .await
    }

    /// Search the Discogs database.
    pub async fn search(
        &self,
        params: &SearchParams,
        pagination: &PaginationParams,
    ) -> Result<Paginated<SearchResult>> {
        if self.inner.auth.is_none() {
            return Err(DiscogsError::AuthRequired);
        }

        self.get_paginated("/database/search", pagination, &params.as_query_pairs())
            .await
    }

    /// Download an image from a Discogs image URL, returning the raw bytes.
    pub async fn download_image(&self, url: &str) -> Result<Bytes> {
        self.inner.rate_limiter.acquire().await;

        let response = self.inner.http.get(url).send().await?;
        let status = response.status();

        if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            return Err(DiscogsError::RateLimited);
        }

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(DiscogsError::Api {
                status: status.as_u16(),
                body,
            });
        }

        Ok(response.bytes().await?)
    }

    /// Fetch a release and download its primary cover image.
    ///
    /// Returns `None` if the release has no images. Prefers the "primary"
    /// image, falling back to the first available image.
    pub async fn release_cover_art(&self, id: u64) -> Result<Option<CoverArt>> {
        let release = self.release(id).await?;
        let image = pick_primary_image(&release.images);

        let Some(image) = image else {
            return Ok(None);
        };
        let Some(ref uri) = image.uri else {
            return Ok(None);
        };

        let data = self.download_image(uri).await?;
        Ok(Some(CoverArt {
            data,
            width: image.width,
            height: image.height,
        }))
    }
}

/// Downloaded cover art image.
pub struct CoverArt {
    /// Raw image bytes (typically JPEG).
    pub data: Bytes,
    /// Image width in pixels, if known.
    pub width: Option<u32>,
    /// Image height in pixels, if known.
    pub height: Option<u32>,
}

/// Pick the primary image from a list, falling back to the first one.
fn pick_primary_image(images: &[Image]) -> Option<&Image> {
    images
        .iter()
        .find(|img| img.image_type.as_deref() == Some("primary"))
        .or_else(|| images.first())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_requires_user_agent() {
        assert!(matches!(
            DiscogsClient::builder().build(),
            Err(DiscogsError::Configuration(_))
        ));
    }

    #[test]
    fn builder_rejects_empty_user_agent() {
        assert!(matches!(
            DiscogsClient::builder().user_agent("").build(),
            Err(DiscogsError::Configuration(_))
        ));
    }

    #[test]
    fn builder_succeeds_with_user_agent() {
        let client = DiscogsClient::builder()
            .user_agent("TestApp/1.0")
            .build();
        assert!(client.is_ok());
    }

    fn make_image(image_type: Option<&str>) -> Image {
        Image {
            image_type: image_type.map(String::from),
            uri: Some("http://example.com/img.jpg".into()),
            uri150: None,
            resource_url: None,
            width: None,
            height: None,
        }
    }

    #[test]
    fn pick_primary_image_prefers_primary() {
        let images = vec![make_image(Some("secondary")), make_image(Some("primary"))];
        let picked = pick_primary_image(&images).unwrap();
        assert_eq!(picked.image_type.as_deref(), Some("primary"));
    }

    #[test]
    fn pick_primary_image_falls_back_to_first() {
        let images = vec![make_image(Some("secondary")), make_image(None)];
        let picked = pick_primary_image(&images).unwrap();
        assert_eq!(picked.image_type.as_deref(), Some("secondary"));
    }

    #[test]
    fn pick_primary_image_empty_returns_none() {
        assert!(pick_primary_image(&[]).is_none());
    }
}
