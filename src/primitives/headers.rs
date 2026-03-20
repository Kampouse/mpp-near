//! MPP HTTP Headers

/// WWW-Authenticate header name
pub const WWW_AUTHENTICATE: &str = "WWW-Authenticate";

/// Authorization header name
pub const AUTHORIZATION: &str = "Authorization";

/// Payment-Receipt header name
pub const PAYMENT_RECEIPT: &str = "Payment-Receipt";

/// Content-Digest header name (RFC 9530)
pub const CONTENT_DIGEST: &str = "Content-Digest";

/// Retry-After header name
pub const RETRY_AFTER: &str = "Retry-After";

/// Cache-Control header name
pub const CACHE_CONTROL: &str = "Cache-Control";

/// Idempotency-Key header name
pub const IDEMPOTENCY_KEY: &str = "Idempotency-Key";

/// Cache control value for challenges
pub const CACHE_NO_STORE: &str = "no-store";

/// Cache control value for receipts
pub const CACHE_PRIVATE: &str = "private";

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_header_names() {
        assert_eq!(WWW_AUTHENTICATE, "WWW-Authenticate");
        assert_eq!(AUTHORIZATION, "Authorization");
        assert_eq!(PAYMENT_RECEIPT, "Payment-Receipt");
    }
}
