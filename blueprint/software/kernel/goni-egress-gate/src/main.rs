use axum::{routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use uuid::Uuid;

use goni_policy::{PolicyDecision, PolicyEngine};
use goni_receipts::{Receipt, ReceiptLog};

#[derive(Clone)]
struct AppState {
    policy: Arc<PolicyEngine>,
    receipts: Arc<ReceiptLog>,
}

#[derive(Debug, Deserialize)]
struct FetchRequest {
    url: String,
    method: Option<String>,
    capability_token: String,
}

#[derive(Debug, Serialize)]
struct FetchResponse {
    status: u16,
    body: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let allowlist = std::env::var("GONI_EGRESS_ALLOWLIST").unwrap_or_default();
    let hosts = allowlist
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    let policy = if hosts.is_empty() {
        PolicyEngine::default_deny()
    } else {
        PolicyEngine::allowlist(hosts)
    };

    let receipt_path = std::env::var("GONI_RECEIPTS_FILE").unwrap_or_else(|_| "./receipts.jsonl".into());
    let receipts = ReceiptLog::open(receipt_path)?;

    let state = AppState {
        policy: Arc::new(policy),
        receipts: Arc::new(receipts),
    };

    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/fetch", post(fetch))
        .with_state(state);

    let addr: SocketAddr = "0.0.0.0:8081".parse()?;
    println!("egress gate listening on {addr}");
    axum::Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}

async fn healthz() -> &'static str {
    "ok"
}

async fn fetch(
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<FetchRequest>,
) -> Result<Json<FetchResponse>, (axum::http::StatusCode, String)> {
    let host = req.url.split('/').nth(2).unwrap_or("");
    let decision = state.policy.evaluate_egress(host);

    let receipt = Receipt {
        receipt_id: Uuid::new_v4(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        action_type: "egress".into(),
        policy_decision: match &decision {
            PolicyDecision::Allow => "allow".into(),
            PolicyDecision::Deny(r) => format!("deny:{r}"),
        },
        capability_id: Some(Uuid::new_v5(&Uuid::NAMESPACE_OID, req.capability_token.as_bytes())),
        input_hash: "".into(),
        output_hash: "".into(),
        prev_hash: None,
        chain_hash: "".into(),
    };
    let _ = state.receipts.append(receipt);

    if !matches!(decision, PolicyDecision::Allow) {
        return Err((axum::http::StatusCode::FORBIDDEN, "egress denied".into()));
    }

    let method = req.method.unwrap_or_else(|| "GET".into());
    let client = reqwest::Client::new();
    let resp = client
        .request(method.parse().unwrap_or(reqwest::Method::GET), &req.url)
        .send()
        .await
        .map_err(|e| (axum::http::StatusCode::BAD_GATEWAY, e.to_string()))?;
    let status = resp.status().as_u16();
    let body = resp.text().await.unwrap_or_default();

    Ok(Json(FetchResponse { status, body }))
}
