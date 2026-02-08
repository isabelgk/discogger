use tokio::sync::Mutex;
use tokio::time::{Duration, Instant};

pub struct RateLimiter {
    inner: Mutex<RateLimiterInner>,
}

struct RateLimiterInner {
    max_tokens: u32,
    tokens: f64,
    last_refill: Instant,
    refill_rate: f64, // tokens per second
}

impl RateLimiter {
    /// Create a rate limiter with `max_per_minute` requests allowed per minute.
    pub fn new(max_per_minute: u32) -> Self {
        Self {
            inner: Mutex::new(RateLimiterInner {
                max_tokens: max_per_minute,
                tokens: max_per_minute as f64,
                last_refill: Instant::now(),
                refill_rate: max_per_minute as f64 / 60.0,
            }),
        }
    }

    /// Wait until a token is available, then consume it.
    pub async fn acquire(&self) {
        loop {
            let wait = {
                let mut inner = self.inner.lock().await;
                inner.refill();
                if inner.tokens >= 1.0 {
                    inner.tokens -= 1.0;
                    return;
                }
                // Calculate how long to wait for one token
                Duration::from_secs_f64(1.0 / inner.refill_rate)
            };
            tokio::time::sleep(wait).await;
        }
    }

    /// Sync the limiter with server-reported usage from the
    /// `X-Discogs-Ratelimit-Used` and `X-Discogs-Ratelimit` headers.
    pub async fn sync_from_headers(&self, used: u32, limit: u32) {
        let mut inner = self.inner.lock().await;
        inner.max_tokens = limit;
        inner.refill_rate = limit as f64 / 60.0;
        let remaining = limit.saturating_sub(used);
        inner.tokens = remaining as f64;
        inner.last_refill = Instant::now();
    }
}

impl RateLimiterInner {
    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        self.tokens = (self.tokens + elapsed * self.refill_rate).min(self.max_tokens as f64);
        self.last_refill = now;
    }
}
