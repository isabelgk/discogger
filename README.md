# discogger

Async Rust client for the [Discogs API](https://www.discogs.com/developers).

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
discogger = { git = "ssh://git@github.com/isabelgk/discogger.git" }
```

Add to your cargo config:
```toml
[net]
git-fetch-with-cli = true
```

### Basic example

```rust
use discogger::{DiscogsClient, PaginationParams, SearchParams, SearchType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DiscogsClient::builder()
        .user_agent("MyApp/1.0")
        .personal_token("your_token_here")
        .build()?;

    // Get an artist
    let artist = client.artist(108713).await?;
    println!("{}", artist.name);

    // Get a release
    let release = client.release(249504).await?;
    println!("{:?}", release.title);

    // Paginated artist releases
    let page = PaginationParams::new(1, 25);
    let releases = client.artist_releases(108713, &page).await?;
    for r in &releases.items {
        println!("{:?} ({:?})", r.title, r.year);
    }
    if releases.has_next() {
        let next = client.artist_releases(108713, &releases.next_page_params().unwrap()).await?;
        // ...
    }

    // Search (requires authentication)
    let results = client.search(
        &SearchParams::new().query("aphex twin").search_type(SearchType::Artist),
        &PaginationParams::default(),
    ).await?;
    for r in &results.items {
        println!("{:?}", r.title);
    }

    Ok(())
}
```

## Authentication

**Personal token** (simplest — get one at [Discogs settings](https://www.discogs.com/settings/developers)):

```rust
DiscogsClient::builder()
    .user_agent("MyApp/1.0")
    .personal_token("token")
    .build()?
```

**OAuth 1.0a:**

```rust
DiscogsClient::builder()
    .user_agent("MyApp/1.0")
    .oauth("consumer_key", "consumer_secret", "token", "token_secret")
    .build()?
```

## API

| Method | Returns |
|---|---|
| `client.artist(id)` | `Artist` |
| `client.artist_releases(id, &pagination)` | `Paginated<ArtistRelease>` |
| `client.release(id)` | `Release` |
| `client.label(id)` | `Label` |
| `client.label_releases(id, &pagination)` | `Paginated<LabelRelease>` |
| `client.master(id)` | `MasterRelease` |
| `client.master_versions(id, &pagination)` | `Paginated<MasterVersion>` |
| `client.search(&params, &pagination)` | `Paginated<SearchResult>` |

## Rate limiting

Built-in token-bucket rate limiter (60 req/min authenticated, 25 req/min unauthenticated). Automatically syncs with server-reported usage via response headers.
