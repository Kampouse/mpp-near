//! Client-side NEAR payment provider for MPP

mod provider;
mod middleware;
mod signer;

pub use provider::{NearProvider, NearConfig};
pub use middleware::PaymentMiddleware;
pub use signer::{NearSigner, SignerError};

#[cfg(feature = "intents")]
mod intents;

#[cfg(feature = "intents")]
pub use intents::{IntentsProvider, IntentsConfig, PaymentCheck, TokenInfo, SwapResult};
