//! Tool syscall boundary (MVP stub).
//!
//! In Goni OS, tools are not ad hoc functions; they are capability-scoped syscalls.
//! This crate defines the execution envelope and audit hooks.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

/// Minimal syscall envelope.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub tool_id: String,
    pub args: serde_json::Value,
    pub agent_id: Uuid,
    pub capability_token_id: Uuid,
    pub state_snapshot_id: Uuid,
    pub policy_hash: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub ok: bool,
    pub output: serde_json::Value,
}

impl ToolCall {
    pub fn args_hash(&self) -> [u8; 32] {
        let mut h = Sha256::new();
        h.update(self.args.to_string().as_bytes());
        h.finalize().into()
    }
}

/// Stub executor: checks capability presence and records an audit entry.
///
/// Enforcement is intentionally minimal in MVP: the kernel policy engine is the source of truth.
pub struct ToolExecutor {
    pub data_plane: std::sync::Arc<dyn goni_store::DataPlane>,
}

impl ToolExecutor {
    pub fn new(data_plane: std::sync::Arc<dyn goni_store::DataPlane>) -> Self {
        Self { data_plane }
    }

    pub async fn execute(&self, call: ToolCall) -> anyhow::Result<ToolResult> {
        // TODO: enforce capability token against tool_id and scopes.
        let _ = call; // placeholder
        Ok(ToolResult {
            ok: false,
            output: serde_json::json!({"error": "tool executor not implemented"}),
        })
    }
}
