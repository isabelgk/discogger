use base64::Engine;
use hmac::{Hmac, Mac};
use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};
use rand::Rng;
use sha1::Sha1;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Characters that must be percent-encoded in OAuth parameters (RFC 5849).
const OAUTH_ENCODE_SET: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'.')
    .remove(b'_')
    .remove(b'~');

fn percent_encode(s: &str) -> String {
    utf8_percent_encode(s, OAUTH_ENCODE_SET).to_string()
}

#[derive(Clone, Debug)]
pub enum Auth {
    PersonalToken(String),
    OAuth {
        consumer_key: String,
        consumer_secret: String,
        token: String,
        token_secret: String,
    },
}

impl Auth {
    /// Apply authentication to a request builder.
    /// For PersonalToken, adds an Authorization header.
    /// For OAuth, computes the HMAC-SHA1 signature and adds the Authorization header.
    pub fn apply(
        &self,
        builder: reqwest::RequestBuilder,
        method: &str,
        url: &str,
    ) -> reqwest::RequestBuilder {
        match self {
            Auth::PersonalToken(token) => {
                builder.header("Authorization", format!("Discogs token={token}"))
            }
            Auth::OAuth {
                consumer_key,
                consumer_secret,
                token,
                token_secret,
            } => {
                let header = build_oauth_header(
                    consumer_key,
                    consumer_secret,
                    token,
                    token_secret,
                    method,
                    url,
                );
                builder.header("Authorization", header)
            }
        }
    }
}

fn build_oauth_header(
    consumer_key: &str,
    consumer_secret: &str,
    token: &str,
    token_secret: &str,
    method: &str,
    url: &str,
) -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();

    let nonce = generate_nonce();

    let mut params = BTreeMap::new();
    params.insert("oauth_consumer_key", consumer_key.to_string());
    params.insert("oauth_nonce", nonce.clone());
    params.insert("oauth_signature_method", "HMAC-SHA1".to_string());
    params.insert("oauth_timestamp", timestamp.clone());
    params.insert("oauth_token", token.to_string());
    params.insert("oauth_version", "1.0".to_string());

    // Parse query params from URL and include them in the signature base
    let (base_url, query_params) = split_url(url);
    for (k, v) in &query_params {
        params.insert(k, v.clone());
    }

    // Build the parameter string (sorted by key)
    let param_string: String = params
        .iter()
        .map(|(k, v)| format!("{}={}", percent_encode(k), percent_encode(v)))
        .collect::<Vec<_>>()
        .join("&");

    // Build the signature base string
    let base_string = format!(
        "{}&{}&{}",
        method.to_uppercase(),
        percent_encode(&base_url),
        percent_encode(&param_string),
    );

    // HMAC-SHA1
    let signing_key = format!(
        "{}&{}",
        percent_encode(consumer_secret),
        percent_encode(token_secret)
    );
    let mut mac = Hmac::<Sha1>::new_from_slice(signing_key.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(base_string.as_bytes());
    let signature = base64::engine::general_purpose::STANDARD.encode(mac.finalize().into_bytes());

    // Build the Authorization header
    format!(
        "OAuth oauth_consumer_key=\"{}\", oauth_nonce=\"{}\", oauth_signature=\"{}\", oauth_signature_method=\"HMAC-SHA1\", oauth_timestamp=\"{}\", oauth_token=\"{}\", oauth_version=\"1.0\"",
        percent_encode(consumer_key),
        percent_encode(&nonce),
        percent_encode(&signature),
        percent_encode(&timestamp),
        percent_encode(token),
    )
}

fn generate_nonce() -> String {
    let mut rng = rand::rng();
    let bytes: [u8; 16] = rng.random();
    hex_encode(&bytes)
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Split a URL into the base (scheme + authority + path) and query parameters.
fn split_url(url: &str) -> (String, Vec<(&str, String)>) {
    if let Some(idx) = url.find('?') {
        let base = &url[..idx];
        let query = &url[idx + 1..];
        let params: Vec<(&str, String)> = query
            .split('&')
            .filter_map(|pair| {
                let mut parts = pair.splitn(2, '=');
                let key = parts.next()?;
                let value = parts.next().unwrap_or("");
                Some((key, value.to_string()))
            })
            .collect();
        (base.to_string(), params)
    } else {
        (url.to_string(), Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percent_encode() {
        assert_eq!(percent_encode("hello world"), "hello%20world");
        assert_eq!(percent_encode("foo+bar"), "foo%2Bbar");
        assert_eq!(percent_encode("test~ok"), "test~ok");
    }

    #[test]
    fn test_split_url() {
        let (base, params) = split_url("https://api.discogs.com/artists/1?page=2&per_page=50");
        assert_eq!(base, "https://api.discogs.com/artists/1");
        assert_eq!(params.len(), 2);
        assert_eq!(params[0], ("page", "2".to_string()));
        assert_eq!(params[1], ("per_page", "50".to_string()));
    }

    #[test]
    fn test_split_url_no_query() {
        let (base, params) = split_url("https://api.discogs.com/artists/1");
        assert_eq!(base, "https://api.discogs.com/artists/1");
        assert!(params.is_empty());
    }

    #[test]
    fn test_oauth_signature() {
        // Verify that the OAuth signing produces a valid Authorization header
        let header = build_oauth_header(
            "consumer_key",
            "consumer_secret",
            "token",
            "token_secret",
            "GET",
            "https://api.discogs.com/artists/1",
        );
        assert!(header.starts_with("OAuth "));
        assert!(header.contains("oauth_consumer_key=\"consumer_key\""));
        assert!(header.contains("oauth_signature_method=\"HMAC-SHA1\""));
        assert!(header.contains("oauth_token=\"token\""));
        assert!(header.contains("oauth_version=\"1.0\""));
        assert!(header.contains("oauth_signature="));
        assert!(header.contains("oauth_nonce="));
        assert!(header.contains("oauth_timestamp="));
    }
}
