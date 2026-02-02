use std::pin::Pin;

use async_trait::async_trait;
use futures_util::{stream, StreamExt};
use goni_types::LlmRequest;
use serde::{Deserialize, Serialize};

type DynStream = Pin<Box<dyn futures_core::Stream<Item = Result<crate::LlmToken, crate::LlmError>> + Send>>;

use crate::{LlmEngine, LlmError, LlmToken, TokenStream};

#[derive(Serialize)]
struct OpenAIChatRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    max_tokens: Option<u32>,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatCompletionChunk {
    choices: Vec<ChatChoiceDelta>,
}

#[derive(Deserialize)]
struct ChatChoiceDelta {
    delta: Delta,
    #[serde(default)]
    index: usize,
}

#[derive(Deserialize)]
struct Delta {
    #[serde(default)]
    content: String,
}

/// Simple HTTP LLM engine that calls a vLLM OpenAI-compatible endpoint.
pub struct HttpVllmEngine {
    client: reqwest::Client,
    base_url: String,
    model: String,
    deterministic: bool,
    seed: Option<u64>,
}

impl HttpVllmEngine {
    pub fn new(base_url: String, model: String, deterministic: bool, seed: Option<u64>) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url,
            model,
            deterministic,
            seed,
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
            stream: true,
            seed: if self.deterministic { self.seed } else { None },
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

        let stream_body = resp.bytes_stream();
        let mut token_id: u32 = 0;
        let s = stream_body.filter_map(move |chunk_res| {
            let mut token_id_local = token_id;
            token_id += 1;
            async move {
                match chunk_res {
                    Ok(bytes) => {
                        // vLLM SSE chunks are lines prefixed with "data: "
                        let text = String::from_utf8_lossy(&bytes);
                        let mut out_tokens = Vec::new();
                        for line in text.lines() {
                            let line = line.trim();
                            if line.is_empty() || line == "data:" {
                                continue;
                            }
                            let line = line.trim_start_matches("data: ");
                            if line == "[DONE]" {
                                continue;
                            }
                            if let Ok(chunk) = serde_json::from_str::<ChatCompletionChunk>(line) {
                                for choice in chunk.choices {
                                    if !choice.delta.content.is_empty() {
                                        out_tokens.push(Ok(LlmToken {
                                            token_id: token_id_local,
                                            text: choice.delta.content.clone(),
                                        }));
                                        token_id_local += 1;
                                    }
                                }
                            }
                        }
                        if out_tokens.is_empty() {
                            None
                        } else {
                            // emit tokens sequentially
                            let stream = stream::iter(out_tokens);
                            Some(stream)
                        }
                    }
                    Err(e) => Some(stream::iter(vec![Err(LlmError {
                        message: format!("stream error: {e}"),
                    })])),
                }
            }
        });

        // Flatten the stream of streams
        let flat_stream = s
            .map(|maybe_stream| maybe_stream.unwrap_or_else(|| stream::empty()))
            .flatten();

        Ok(Box::pin(flat_stream) as TokenStream)
    }
}
