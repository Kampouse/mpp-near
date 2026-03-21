//! Client-side NEAR payment provider for MPP

mod provider;
mod middleware;
mod signer;
mod agent_client;

pub use provider::{NearProvider, NearConfig};
pub use middleware::PaymentMiddleware;
pub use signer::{NearSigner, SignerError};
pub use agent_client::{AgentClient, AgentError, BudgetConfig, Challenge402};

#[cfg(feature = "intents")]
mod intents;

#[cfg(feature = "intents")]
pub use intents::{IntentsProvider, IntentsConfig, PaymentCheck, TokenInfo, SwapResult};
