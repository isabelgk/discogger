#![cfg(feature = "blocking")]

use discogger::blocking::DiscogsClient;
use discogger::{DiscogsError, PaginationParams};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn json(status: u16, body: &str) -> ResponseTemplate {
    ResponseTemplate::new(status)
        .insert_header("content-type", "application/json")
        .set_body_string(body)
}

// The blocking client creates its own tokio runtime internally. To avoid
// "cannot start a runtime from within a runtime" we run it on a fresh OS
// thread. The outer #[tokio::test] runtime is only used to drive wiremock.

fn run_blocking<F, T>(f: F) -> T
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    std::thread::spawn(f).join().unwrap()
}

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

#[tokio::test]
async fn artist_blocking() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/artists/45"))
        .respond_with(json(200, r#"{"id": 45, "name": "Aphex Twin"}"#))
        .mount(&server)
        .await;

    let base_url = server.uri();
    let artist = run_blocking(move || {
        DiscogsClient::builder()
            .user_agent("test/1.0")
            .base_url(base_url)
            .build()
            .unwrap()
            .artist(45)
            .unwrap()
    });

    assert_eq!(artist.id, 45);
    assert_eq!(artist.name, "Aphex Twin");
}

#[tokio::test]
async fn release_blocking() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/releases/249504"))
        .respond_with(json(
            200,
            r#"{"id": 249504, "title": "Never Gonna Give You Up", "year": 1987}"#,
        ))
        .mount(&server)
        .await;

    let base_url = server.uri();
    let release = run_blocking(move || {
        DiscogsClient::builder()
            .user_agent("test/1.0")
            .base_url(base_url)
            .build()
            .unwrap()
            .release(249504)
            .unwrap()
    });

    assert_eq!(release.id, 249504);
    assert_eq!(release.title.as_deref(), Some("Never Gonna Give You Up"));
}

#[tokio::test]
async fn rate_limited_blocking() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/artists/1"))
        .respond_with(ResponseTemplate::new(429))
        .mount(&server)
        .await;

    let base_url = server.uri();
    let err = run_blocking(move || {
        DiscogsClient::builder()
            .user_agent("test/1.0")
            .base_url(base_url)
            .build()
            .unwrap()
            .artist(1)
            .unwrap_err()
    });

    assert!(matches!(err, DiscogsError::RateLimited));
}

#[tokio::test]
async fn search_requires_auth_blocking() {
    let server = MockServer::start().await;
    let base_url = server.uri();

    let err = run_blocking(move || {
        DiscogsClient::builder()
            .user_agent("test/1.0")
            .base_url(base_url)
            .build()
            .unwrap()
            .search(
                &discogger::SearchParams::new(),
                &PaginationParams::default(),
            )
            .unwrap_err()
    });

    assert!(matches!(err, DiscogsError::AuthRequired));
}
