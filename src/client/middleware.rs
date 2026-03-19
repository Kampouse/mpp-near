//! HTTP middleware for automatic 402 payment handling

use async_trait::async_trait;
use reqwest::{Request, Response};
use reqwest_middleware::{Middleware, Next};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::client::NearProvider;
use crate::types::NearChallenge;

/// Payment middleware state
#[derive(Debug, Default)]
struct MiddlewareState {
    pending_payments: std::collections::HashMap<String, NearChallenge>,
}

/// Middleware that automatically handles HTTP 402 responses
pub struct PaymentMiddleware {
    provider: Arc<NearProvider>,
    max_retries: usize,
    state: Arc<RwLock<MiddlewareState>>,
}

impl PaymentMiddleware {
    /// Create new payment middleware
    pub fn new(provider: NearProvider) -> Self {
        Self {
            provider: Arc::new(provider),
            max_retries: 3,
            state: Arc::new(RwLock::new(MiddlewareState::default())),
        }
    }
    
    /// Create with arc provider
    pub fn with_provider(provider: Arc<NearProvider>) -> Self {
        Self {
            provider,
            max_retries: 3,
            state: Arc::new(RwLock::new(MiddlewareState::default())),
        }
    }
    
    /// Set max retries
    pub fn with_max_retries(mut self, max: usize) -> Self {
        self.max_retries = max;
        self
    }
    
    /// Handle 402 response
    async fn handle_402(&self, response: Response, request: &Request) -> crate::Result<Response> {
        // Parse challenge from response
        let challenge: NearChallenge = response.json().await?;
        
        tracing::info!(
            "Received 402 challenge: {} (amount: {})",
            challenge.challenge_id,
            challenge.amount
        );
        
        // Pay the challenge
        let credential = self.provider.pay_challenge(&challenge).await?;
        
        // Build retry request
        let client = reqwest::Client::new();
        let retry_request = client
            .request(request.method().clone(), request.url().clone())
            .header("Authorization", format!(
                "Payment {}",
                serde_json::to_string(&credential)?
            ));
        
        let retry_response = retry_request.send().await?;
        
        Ok(retry_response)
    }
}

#[async_trait]
impl Middleware for PaymentMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut http::Extensions,
        next: Next<'_>,
    ) -> reqwest_middleware::Result<Response> {
        let mut retries = 0;
        
        loop {
            // Send request
            let response = next.clone().run(req.try_clone().unwrap(), extensions).await;
            
            match response {
                Ok(resp) => {
                    // Check for 402
                    if resp.status() == 402 && retries < self.max_retries {
                        retries += 1;
                        
                        // Handle payment
                        match self.handle_402(resp, &req).await {
                            Ok(paid_response) => return Ok(paid_response),
                            Err(e) => {
                                tracing::error!("Payment failed: {}", e);
                                return Err(anyhow::anyhow!("Payment failed: {}", e).into());
                            }
                        }
                    } else {
                        return Ok(resp);
                    }
                }
                Err(e) => return Err(e),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_middleware_creation() {
        // Would need full integration test with real provider
        assert!(true);
    }
}
