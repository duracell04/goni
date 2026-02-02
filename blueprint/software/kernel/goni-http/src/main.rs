use std::net::SocketAddr;
use std::sync::Arc;

use axum::{extract::State, routing::{get, post}, Json, Router};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;
use uuid::Uuid;
use sha2::{Digest, Sha256};

use goni_core::GoniKernel;
use goni_context::{FacilityLocationSelector, NullKvPager};
use goni_infer::{HttpVllmEngine, NullLlmEngine};
use goni_receipts::{Receipt, ReceiptLog};
use goni_router::{ConfigRouter, NullRouter, Router};
use goni_sched::InMemoryScheduler;
use goni_store::{InMemorySpineDataPlane, MultiDataPlane, NullDataPlane, QdrantDataPlane};
use goni_types::TaskClass;

#[derive(Clone)]
struct AppState {
    kernel: Arc<GoniKernel>,
    receipts: Arc<ReceiptLog>,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<Message>,
    #[serde(default)]
    max_tokens: Option<u32>,
    #[serde(default)]
    stream: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct ChatCompletionResponse {
    id: String,
    object: String,
    choices: Vec<Choice>,
}

#[derive(Debug, Serialize)]
struct Choice {
    index: u32,
    message: Message,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Read env for vLLM and model
    let llm_url = std::env::var("LLM_LOCAL_URL")
        .unwrap_or_else(|_| "http://localhost:8000/v1".into());
    let llm_model = std::env::var("LLM_MODEL")
        .unwrap_or_else(|_| "mistralai/Mistral-7B-Instruct-v0.3".into());
    let llm_deterministic = std::env::var("LLM_DETERMINISTIC")
        .map(|v| v == "1" || v.to_lowercase() == "true")
        .unwrap_or(false);
    let llm_seed = std::env::var("LLM_SEED")
        .ok()
        .and_then(|v| v.parse::<u64>().ok());

    if llm_deterministic {
        println!(
            "LLM deterministic preset enabled (seed = {:?})",
            llm_seed
        );
    }

    // Data plane:
    // - an in-memory "spine" for control/state/audit tables (append-only during MVP)
    // - an optional Qdrant backend for RAG search/ingest
    let spine_plane: Arc<dyn goni_store::DataPlane> = Arc::new(InMemorySpineDataPlane::new());
    let rag_plane: Arc<dyn goni_store::DataPlane> = match std::env::var("QDRANT_HTTP_URL") {
        Ok(url) if !url.is_empty() => Arc::new(QdrantDataPlane::new(url)),
        _ => Arc::new(NullDataPlane),
    };
    let data_plane: Arc<dyn goni_store::DataPlane> = Arc::new(MultiDataPlane::new(spine_plane, rag_plane));
    let context_selector = Arc::new(FacilityLocationSelector::new(0.3));
    let kv_pager = Arc::new(NullKvPager);
    let scheduler = Arc::new(InMemoryScheduler::new());
    let router: Arc<dyn Router> = match std::env::var("GONI_ROUTER_CONFIG") {
        Ok(path) => ConfigRouter::from_path(path)
            .map(|r| Arc::new(r) as Arc<dyn Router>)
            .unwrap_or_else(|_| Arc::new(NullRouter)),
        Err(_) => Arc::new(NullRouter),
    };
    let use_stub = std::env::var("LLM_STUB")
        .map(|v| v == "1" || v.to_lowercase() == "true")
        .unwrap_or(false);
    let llm_engine: Arc<dyn goni_infer::LlmEngine> = if use_stub {
        Arc::new(NullLlmEngine)
    } else {
        Arc::new(HttpVllmEngine::new(
            llm_url,
            llm_model,
            llm_deterministic,
            llm_seed,
        ))
    };

    let receipt_path = std::env::var("GONI_RECEIPTS_FILE")
        .unwrap_or_else(|_| "./receipts.jsonl".into());
    let receipts = Arc::new(ReceiptLog::open(receipt_path)?);

    let kernel = Arc::new(GoniKernel::new(
        data_plane,
        context_selector,
        kv_pager,
        scheduler,
        router,
        llm_engine,
    ));

    // Start the scheduler executor loop (LLM-as-interrupt handler).
    // This preserves the architectural boundary: request submission ≠ solver execution.
    let kernel_exec = Arc::clone(&kernel);
    tokio::spawn(async move {
        kernel_exec.run_scheduler_loop().await;
    });

    let app_state = AppState { kernel, receipts };

    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/v1/chat/completions", post(chat_completions))
        .with_state(app_state)
        .layer(TraceLayer::new_for_http());

    let addr: SocketAddr = "0.0.0.0:7000".parse()?;
    println!("Goni orchestrator HTTP server listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn chat_completions(
    State(state): State<AppState>,
    Json(req): Json<ChatCompletionRequest>,
) -> Result<Json<ChatCompletionResponse>, (axum::http::StatusCode, String)> {
    // Flatten user messages into one prompt for MVP.
    let mut prompt = String::new();
    for m in req.messages.iter().filter(|m| m.role == "user") {
        if !prompt.is_empty() {
            prompt.push_str("\n");
        }
        prompt.push_str(&m.content);
    }

    let max_tokens = req.max_tokens.unwrap_or(256);

    let mut stream = state
        .kernel
        .handle_user_query(&prompt, TaskClass::Interactive)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut full_text = String::new();
    while let Some(tok_res) = stream.next().await {
        let tok = tok_res.map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("LLM token error: {:?}", e),
            )
        })?;
        full_text.push_str(&tok.text);
        if full_text.chars().count() >= max_tokens as usize {
            break;
        }
    }

    let resp = ChatCompletionResponse {
        id: Uuid::new_v4().to_string(),
        object: "chat.completion".into(),
        choices: vec![Choice {
            index: 0,
            message: Message {
                role: "assistant".into(),
                content: full_text,
            },
        }],
    };

    let input_hash = format!("{:x}", Sha256::digest(prompt.as_bytes()));
    let output_hash = format!("{:x}", Sha256::digest(resp.choices[0].message.content.as_bytes()));
    let receipt = Receipt {
        receipt_id: Uuid::new_v4(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        action_type: "chat.completion".into(),
        policy_decision: "allow".into(),
        capability_id: None,
        input_hash,
        output_hash,
        prev_hash: None,
        chain_hash: String::new(),
    };
    let _ = state.receipts.append(receipt);

    Ok(Json(resp))
}

async fn healthz() -> &'static str {
    "ok"
}
