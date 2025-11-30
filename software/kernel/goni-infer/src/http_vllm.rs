use std::pin::Pin;

use async_trait::async_trait;
use futures_util::stream;
use goni_types::LlmRequest;
use serde::{Deserialize, Serialize};

use crate::{LlmEngine, LlmError, LlmToken, TokenStream};

#[derive(Serialize)]
struct OpenAIChatRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    max_tokens: Option<u32>,
}

#[derive(Serialize, Deserialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAIChatResponse {
    choices: Vec<OpenAIChoice>,
}

#[derive(Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessage,
}

/// Simple HTTP LLM engine that calls a vLLM OpenAI-compatible endpoint.
pub struct HttpVllmEngine {
    client: reqwest::Client,
    base_url: String,
    model: String,
}

impl HttpVllmEngine {
    pub fn new(base_url: String, model: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url,
            model,
        }
    }
}

#[async_trait]
impl LlmEngine for HttpVllmEngine {
    async fn generate(
        &self,
        req: LlmRequest,
    ) -> Result<TokenStream, LlmError> {
        let url = format!("{}/chat/completions", self.base_url);

        let body = OpenAIChatRequest {
            model: self.model.clone(),
            messages: vec![OpenAIMessage {
                role: "user".into(),
                content: req.prompt,
            }],
            max_tokens: Some(req.max_tokens as u32),
        };

        let resp = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| LlmError {
                message: format!("HTTP error: {e}"),
            })?;

        if !resp.status().is_success() {
            return Err(LlmError {
                message: format!("HTTP status: {}", resp.status()),
            });
        }

        let parsed: OpenAIChatResponse = resp.json().await.map_err(|e| LlmError {
            message: format!("JSON error: {e}"),
        })?;

        let text = parsed
            .choices
            .get(0)
            .map(|c| c.message.content.clone())
            .unwrap_or_default();

        let stream = stream::once(async move {
            Ok(LlmToken {
                token_id: 0,
                text,
            })
        });

        Ok(Box::pin(stream) as Pin<Box<dyn futures_core::Stream<Item = Result<LlmToken, LlmError>> + Send>>)
    }
}
