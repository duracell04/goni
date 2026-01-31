//! Tool syscall boundary (MVP stub).
//!
//! In Goni OS, tools are not ad hoc functions; they are capability-scoped syscalls.
//! This crate defines the execution envelope and audit hooks.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use goni_policy::{BudgetLedger, CapabilityToken, PolicyDecision, PolicyEngine};
use goni_receipts::{Receipt, ReceiptLog};

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
    pub policy: PolicyEngine,
    pub receipts: ReceiptLog,
}

impl ToolExecutor {
    pub fn new(
        data_plane: std::sync::Arc<dyn goni_store::DataPlane>,
        policy: PolicyEngine,
        receipts: ReceiptLog,
    ) -> Self {
        Self { data_plane, policy, receipts }
    }

    pub async fn execute(
        &self,
        call: ToolCall,
        token: CapabilityToken,
        ledger: &mut BudgetLedger,
    ) -> anyhow::Result<ToolResult> {
        let decision = self.policy.evaluate_tool(&token, &call.tool_id, ledger);

        let receipt = Receipt {
            receipt_id: Uuid::new_v4(),
            timestamp: format!("{:?}", std::time::SystemTime::now()),
            action_type: "toolcall".into(),
            policy_decision: match &decision {
                PolicyDecision::Allow => "allow".into(),
                PolicyDecision::Deny(r) => format!("deny:{r}"),
            },
            capability_id: Some(call.capability_token_id),
            input_hash: hex::encode(call.args_hash()),
            output_hash: hex::encode([0u8; 32]),
            prev_hash: None,
            chain_hash: String::new(),
        };
        let _ = self.receipts.append(receipt);

        if !matches!(decision, PolicyDecision::Allow) {
            return Ok(ToolResult {
                ok: false,
                output: serde_json::json!({"error": "capability denied"}),
            });
        }

        Ok(ToolResult {
            ok: false,
            output: serde_json::json!({"error": "tool executor not implemented"}),
        })
    }
}
