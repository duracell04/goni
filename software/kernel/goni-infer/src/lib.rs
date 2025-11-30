use std::pin::Pin;

use async_trait::async_trait;
use futures_core::Stream;
use goni_types::LlmRequest;

pub mod http_vllm;
pub use http_vllm::HttpVllmEngine;

#[derive(Clone, Debug)]
pub struct LlmToken {
    pub token_id: u32,
    pub text: String,
}

pub type TokenStream =
    Pin<Box<dyn Stream<Item = Result<LlmToken, LlmError>> + Send>>;

#[derive(Debug)]
pub struct LlmError {
    pub message: String,
}

#[async_trait]
pub trait LlmEngine: Send + Sync {
    async fn generate(
        &self,
        req: LlmRequest,
    ) -> Result<TokenStream, LlmError>;
}

/// Dummy implementation that yields no tokens.
pub struct NullLlmEngine;

#[async_trait]
impl LlmEngine for NullLlmEngine {
    async fn generate(
        &self,
        _req: LlmRequest,
    ) -> Result<TokenStream, LlmError> {
        use futures_util::stream;

        Ok(Box::pin(stream::empty()))
    }
}
