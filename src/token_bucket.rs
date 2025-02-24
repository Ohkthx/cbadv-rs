//! Bucket for managing and consuming tokens to prevent API rate limiting.

use std::time::{Duration, Instant};
use tokio::time::sleep as async_sleep;

use crate::constants::ratelimits;

/// Rate Limits for REST and WebSocket requests.
///
/// # Endpoint / Reference
///
/// * REST: <https://docs.cloud.coinbase.com/advanced-trade-api/docs/rest-api-rate-limits>
/// * WebSocket: <https://docs.cloud.coinbase.com/advanced-trade-api/docs/ws-rate-limits>
pub(crate) struct RateLimits {}
impl RateLimits {
    /// Maximum amount of tokens per bucket.
    const PUBLIC_REST_MAX_TOKENS: f64 = ratelimits::PUBLIC_REST_REFRESH_RATE;
    const SECURE_REST_MAX_TOKENS: f64 = ratelimits::SECURE_REST_REFRESH_RATE;
    const PUBLIC_WEBSOCKET_MAX_TOKENS: f64 = ratelimits::PUBLIC_WEBSOCKET_REFRESH_RATE;
    const SECURE_WEBSOCKET_MAX_TOKENS: f64 = ratelimits::SECURE_WEBSOCKET_REFRESH_RATE;

    /// Amount of tokens refreshed per second.
    ///
    /// # Arguments
    ///
    /// * `is_rest` - Requester is REST Client, true, otherwise false.
    /// * `is_public` - Requester is Public Client, true, otherwise false.
    pub(crate) fn refresh_rate(is_rest: bool, is_public: bool) -> f64 {
        if is_rest {
            if is_public {
                ratelimits::PUBLIC_REST_REFRESH_RATE
            } else {
                ratelimits::SECURE_REST_REFRESH_RATE
            }
        } else if is_public {
            ratelimits::PUBLIC_WEBSOCKET_REFRESH_RATE
        } else {
            ratelimits::SECURE_WEBSOCKET_REFRESH_RATE
        }
    }

    /// Maximum amount of tokens for a bucket.
    ///
    /// # Arguments
    ///
    /// * `is_rest` - Requester is REST Client, true, otherwise false.
    /// * `is_public` - Requester is Public Client, true, otherwise false.
    pub(crate) fn max_tokens(is_rest: bool, is_public: bool) -> f64 {
        if is_rest {
            if is_public {
                RateLimits::PUBLIC_REST_MAX_TOKENS
            } else {
                RateLimits::SECURE_REST_MAX_TOKENS
            }
        } else if is_public {
            RateLimits::PUBLIC_WEBSOCKET_MAX_TOKENS
        } else {
            RateLimits::SECURE_WEBSOCKET_MAX_TOKENS
        }
    }
}

/// Contains and tracks token usage for rate limits.
#[derive(Debug, Clone)]
pub(crate) struct TokenBucket {
    /// Maximum amount of tokens allowed in the bucket at a time.
    max_tokens: f64,
    /// Amount of tokens replenished per second.
    refill_rate: f64,
    /// Last time a token was consumed.
    last_consumption: Instant,
    /// Amount of current token in the bucket.
    tokens: f64,
}

impl TokenBucket {
    /// Creates a new instance of the bucket.
    ///
    /// # Arguments
    ///
    /// * `max_tokens` - Maximum amount of tokens allowed in the bucket.
    /// * `refill_rate` - How many tokens per second are refreshed.
    pub(crate) fn new(max_tokens: f64, refill_rate: f64) -> Self {
        Self {
            max_tokens,
            refill_rate,
            last_consumption: Instant::now(),
            tokens: max_tokens,
        }
    }

    /// Calculates amount of seconds (Duration) until next token is available.
    fn next_token(&self) -> Duration {
        if self.tokens >= 1.0 {
            Duration::ZERO
        } else {
            Duration::from_secs_f64((1.0 - self.tokens) / self.refill_rate)
        }
    }

    /// Attempts to consume a token if one is available. This also checks to see if any tokens need to be refilled
    /// in the process.
    fn consume(&mut self) -> bool {
        let now = Instant::now();
        let time_elapsed = now.duration_since(self.last_consumption);

        // Refill the token bucket based on time passed.
        let tokens_to_refill = time_elapsed.as_secs_f64() * self.refill_rate;
        self.tokens = (self.tokens + tokens_to_refill).min(self.max_tokens);

        // Return early if we cannot consume a token.
        if self.tokens < 1.0 {
            false
        } else {
            // Consume token.
            self.tokens -= 1.0;
            self.last_consumption = now;
            true
        }
    }

    /// Blocks until a token is ready and immediately consumes it.
    pub(crate) async fn wait_on(&mut self) {
        while !self.consume() {
            async_sleep(self.next_token()).await;
        }
    }
}
