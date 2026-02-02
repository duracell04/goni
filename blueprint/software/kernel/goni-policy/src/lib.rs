use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Policy {
    pub mode: String,
    pub allowlist: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CapabilityToken {
    pub token_id: Uuid,
    pub scopes: Vec<String>,
    pub expires_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BudgetLedger {
    pub bytes_remaining: i64,
    pub tokens_remaining: i64,
    pub tool_calls_remaining: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum PolicyDecision {
    Allow,
    Deny(String),
}

impl BudgetLedger {
    pub fn new(bytes: i64, tokens: i64, tool_calls: i64) -> Self {
        Self {
            bytes_remaining: bytes,
            tokens_remaining: tokens,
            tool_calls_remaining: tool_calls,
        }
    }

    pub fn debit_tool_call(&mut self) -> Result<(), PolicyDecision> {
        if self.tool_calls_remaining <= 0 {
            return Err(PolicyDecision::Deny("tool_call_budget_exhausted".into()));
        }
        self.tool_calls_remaining -= 1;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct PolicyEngine {
    policy: Policy,
}

impl PolicyEngine {
    pub fn new(policy: Policy) -> Self {
        Self { policy }
    }

    pub fn default_deny() -> Self {
        Self {
            policy: Policy {
                mode: "deny".into(),
                allowlist: Vec::new(),
            },
        }
    }

    pub fn allowlist(allowlist: Vec<String>) -> Self {
        Self {
            policy: Policy {
                mode: "allowlist".into(),
                allowlist,
            },
        }
    }

    pub fn evaluate_tool(
        &self,
        token: &CapabilityToken,
        tool_id: &str,
        ledger: &mut BudgetLedger,
    ) -> PolicyDecision {
        if !token.scopes.iter().any(|s| s == tool_id || s == "*") {
            return PolicyDecision::Deny("scope_not_allowed".into());
        }
        if let Err(decision) = ledger.debit_tool_call() {
            return decision;
        }
        PolicyDecision::Allow
    }

    pub fn evaluate_egress(&self, host: &str) -> PolicyDecision {
        if self.policy.mode == "deny" {
            return PolicyDecision::Deny("egress_denied".into());
        }
        if self.policy.allowlist.iter().any(|h| h == host) {
            PolicyDecision::Allow
        } else {
            PolicyDecision::Deny("host_not_allowed".into())
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum InitiativeDecision {
    Defer,
    Propose,
    Ask,
    Act,
}

pub fn decide_initiative(
    confidence: f32,
    urgency: f32,
    interruptibility: f32,
    reversible: bool,
) -> InitiativeDecision {
    if !reversible {
        return InitiativeDecision::Ask;
    }
    if confidence > 0.8 && urgency > 0.7 && interruptibility > 0.5 {
        InitiativeDecision::Act
    } else if confidence > 0.6 {
        InitiativeDecision::Propose
    } else {
        InitiativeDecision::Defer
    }
}

#[derive(Clone, Debug)]
pub struct MemoryWriteRequest {
    pub evidence_ids: Vec<String>,
    pub receipt_id: Option<Uuid>,
    pub ttl_days: Option<u32>,
    pub uncertain: bool,
}

pub fn evaluate_memory_write(req: &MemoryWriteRequest) -> PolicyDecision {
    if req.evidence_ids.is_empty() {
        return PolicyDecision::Deny("missing_evidence".into());
    }
    if req.receipt_id.is_none() {
        return PolicyDecision::Deny("missing_receipt".into());
    }
    if req.uncertain && req.ttl_days.is_none() {
        return PolicyDecision::Deny("missing_ttl".into());
    }
    PolicyDecision::Allow
}

#[derive(Clone, Debug)]
pub struct RedactionRequest {
    pub profile_present: bool,
    pub plane_ok: bool,
    pub manifest_present: bool,
}

pub fn evaluate_redaction(req: &RedactionRequest) -> PolicyDecision {
    if !req.profile_present {
        return PolicyDecision::Deny("missing_redaction_profile".into());
    }
    if !req.plane_ok {
        return PolicyDecision::Deny("plane_violation".into());
    }
    if !req.manifest_present {
        return PolicyDecision::Deny("missing_manifest".into());
    }
    PolicyDecision::Allow
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initiative_is_deterministic() {
        let a = decide_initiative(0.9, 0.8, 0.6, true);
        let b = decide_initiative(0.9, 0.8, 0.6, true);
        assert_eq!(a, b);
    }

    #[test]
    fn tool_scope_denied() {
        let engine = PolicyEngine::default_deny();
        let token = CapabilityToken {
            token_id: Uuid::new_v4(),
            scopes: vec!["demo.echo".into()],
            expires_at: None,
        };
        let mut ledger = BudgetLedger::new(1, 1, 1);
        let decision = engine.evaluate_tool(&token, "other.tool", &mut ledger);
        assert!(matches!(decision, PolicyDecision::Deny(_)));
    }

    #[test]
    fn memory_write_requires_evidence() {
        let req = MemoryWriteRequest {
            evidence_ids: Vec::new(),
            receipt_id: None,
            ttl_days: None,
            uncertain: false,
        };
        let decision = evaluate_memory_write(&req);
        assert!(matches!(decision, PolicyDecision::Deny(_)));
    }

    #[test]
    fn redaction_requires_profile() {
        let req = RedactionRequest {
            profile_present: false,
            plane_ok: true,
            manifest_present: true,
        };
        let decision = evaluate_redaction(&req);
        assert!(matches!(decision, PolicyDecision::Deny(_)));
    }
}
