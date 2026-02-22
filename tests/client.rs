use discogger::{DiscogsClient, DiscogsError, PaginationParams, SearchParams, SearchType};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn client(base_url: &str) -> DiscogsClient {
    DiscogsClient::builder()
        .user_agent("test/1.0")
        .base_url(base_url)
        .build()
        .unwrap()
}

fn auth_client(base_url: &str) -> DiscogsClient {
    DiscogsClient::builder()
        .user_agent("test/1.0")
        .personal_token("testtoken")
        .base_url(base_url)
        .build()
        .unwrap()
}

fn json(status: u16, body: &str) -> ResponseTemplate {
    ResponseTemplate::new(status)
        .insert_header("content-type", "application/json")
        .set_body_string(body)
}

// --- artist ---

#[tokio::test]
async fn artist_parses_response() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/artists/45"))
        .respond_with(json(200, r#"{"id": 45, "name": "Aphex Twin"}"#))
        .mount(&server)
        .await;

    let artist = client(&server.uri()).artist(45).await.unwrap();
    assert_eq!(artist.id, 45);
    assert_eq!(artist.name, "Aphex Twin");
}

#[tokio::test]
async fn artist_releases_parses_paginated_response() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/artists/45/releases"))
        .respond_with(json(
            200,
            r#"{
                "pagination": {"page": 1, "pages": 2, "per_page": 50, "items": 75},
                "releases": [{"id": 100, "title": "SAW 85-92", "year": 1992, "type": "master"}]
            }"#,
        ))
        .mount(&server)
        .await;

    let page = client(&server.uri())
        .artist_releases(45, &PaginationParams::default())
        .await
        .unwrap();

    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].id, 100);
    assert!(page.has_next());
    assert_eq!(page.total_items(), 75);
}

// --- release ---

#[tokio::test]
async fn release_parses_response() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/releases/249504"))
        .respond_with(json(
            200,
            r#"{"id": 249504, "title": "Never Gonna Give You Up", "year": 1987}"#,
        ))
        .mount(&server)
        .await;

    let release = client(&server.uri()).release(249504).await.unwrap();
    assert_eq!(release.id, 249504);
    assert_eq!(release.title.as_deref(), Some("Never Gonna Give You Up"));
}

// --- label ---

#[tokio::test]
async fn label_parses_response() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/labels/1"))
        .respond_with(json(200, r#"{"id": 1, "name": "Warp Records"}"#))
        .mount(&server)
        .await;

    let label = client(&server.uri()).label(1).await.unwrap();
    assert_eq!(label.id, 1);
    assert_eq!(label.name.as_deref(), Some("Warp Records"));
}

#[tokio::test]
async fn label_releases_parses_paginated_response() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/labels/1/releases"))
        .respond_with(json(
            200,
            r#"{
                "pagination": {"page": 1, "pages": 1, "per_page": 50, "items": 1},
                "releases": [{"id": 200, "title": "Spyra", "artist": "Various", "catno": "WARP001"}]
            }"#,
        ))
        .mount(&server)
        .await;

    let page = client(&server.uri())
        .label_releases(1, &PaginationParams::default())
        .await
        .unwrap();

    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].catno.as_deref(), Some("WARP001"));
    assert!(!page.has_next());
}

// --- master ---

#[tokio::test]
async fn master_parses_response() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/masters/4148"))
        .respond_with(json(
            200,
            r#"{"id": 4148, "title": "Selected Ambient Works 85-92", "year": 1992}"#,
        ))
        .mount(&server)
        .await;

    let master = client(&server.uri()).master(4148).await.unwrap();
    assert_eq!(master.id, 4148);
    assert_eq!(master.year, Some(1992));
}

#[tokio::test]
async fn master_versions_parses_paginated_response() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/masters/4148/versions"))
        .respond_with(json(
            200,
            r#"{
                "pagination": {"page": 1, "pages": 1, "per_page": 50, "items": 2},
                "versions": [
                    {"id": 67896, "title": "SAW 85-92", "year": 1992, "country": "UK"},
                    {"id": 67897, "title": "SAW 85-92", "year": 1993, "country": "US"}
                ]
            }"#,
        ))
        .mount(&server)
        .await;

    let page = client(&server.uri())
        .master_versions(4148, &PaginationParams::default())
        .await
        .unwrap();

    assert_eq!(page.items.len(), 2);
    assert_eq!(page.items[0].country.as_deref(), Some("UK"));
    assert_eq!(page.items[1].country.as_deref(), Some("US"));
}

// --- search ---

#[tokio::test]
async fn search_requires_auth() {
    let server = MockServer::start().await;
    let err = client(&server.uri())
        .search(&SearchParams::new(), &PaginationParams::default())
        .await
        .unwrap_err();
    assert!(matches!(err, DiscogsError::AuthRequired));
}

#[tokio::test]
async fn search_parses_response() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/database/search"))
        .respond_with(json(
            200,
            r#"{
                "pagination": {"page": 1, "pages": 1, "per_page": 50, "items": 1},
                "results": [{"id": 108713, "type": "artist", "title": "Aphex Twin"}]
            }"#,
        ))
        .mount(&server)
        .await;

    let params = SearchParams::new()
        .query("Aphex Twin")
        .search_type(SearchType::Artist);
    let page = auth_client(&server.uri())
        .search(&params, &PaginationParams::default())
        .await
        .unwrap();

    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].id, 108713);
    assert_eq!(page.items[0].result_type.as_deref(), Some("artist"));
}

// --- error handling ---

#[tokio::test]
async fn rate_limited_response_returns_error() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/artists/1"))
        .respond_with(ResponseTemplate::new(429))
        .mount(&server)
        .await;

    let err = client(&server.uri()).artist(1).await.unwrap_err();
    assert!(matches!(err, DiscogsError::RateLimited));
}

#[tokio::test]
async fn api_error_response_captures_status_and_body() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/artists/999"))
        .respond_with(json(404, r#"{"message": "Release not found."}"#))
        .mount(&server)
        .await;

    let err = client(&server.uri()).artist(999).await.unwrap_err();
    match err {
        DiscogsError::Api { status, body } => {
            assert_eq!(status, 404);
            assert!(body.contains("not found"));
        }
        other => panic!("expected Api error, got {other:?}"),
    }
}

// --- download_image / release_cover_art ---

#[tokio::test]
async fn download_image_returns_bytes() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/images/cover.jpg"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "image/jpeg")
                .set_body_bytes(b"FAKEJPEGDATA".to_vec()),
        )
        .mount(&server)
        .await;

    let url = format!("{}/images/cover.jpg", server.uri());
    let bytes = client(&server.uri()).download_image(&url).await.unwrap();
    assert_eq!(&bytes[..], b"FAKEJPEGDATA");
}

#[tokio::test]
async fn release_cover_art_returns_none_when_no_images() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/releases/1"))
        .respond_with(json(200, r#"{"id": 1, "title": "No Cover", "images": []}"#))
        .mount(&server)
        .await;

    let result = client(&server.uri()).release_cover_art(1).await.unwrap();
    assert!(result.is_none());
}

#[tokio::test]
async fn release_cover_art_downloads_primary_image() {
    let server = MockServer::start().await;

    let release_body = format!(
        r#"{{"id": 1, "title": "Has Cover", "images": [{{"type": "primary", "uri": "{}/images/cover.jpg", "width": 300, "height": 300}}]}}"#,
        server.uri()
    );

    Mock::given(method("GET"))
        .and(path("/releases/1"))
        .respond_with(json(200, &release_body))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/images/cover.jpg"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "image/jpeg")
                .set_body_bytes(b"JPEGDATA".to_vec()),
        )
        .mount(&server)
        .await;

    let art = client(&server.uri())
        .release_cover_art(1)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(&art.data[..], b"JPEGDATA");
    assert_eq!(art.width, Some(300));
    assert_eq!(art.height, Some(300));
}
