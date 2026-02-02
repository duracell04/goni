//! Minimal agent packaging primitives.
//!
//! This crate intentionally starts small: it provides a manifest schema and parsing helpers.
//! Enforcement (capabilities/budgets) lives in the kernel policy engine.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

/// Canonical agent manifest format (YAML).
///
/// This mirrors `docs/specs/agent-manifest.md`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentManifest {
    pub id: String,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub triggers: Vec<Trigger>,
    pub permissions: Permissions,
    #[serde(default)]
    pub budgets: Budgets,
    #[serde(default)]
    pub tools: Vec<String>,
    #[serde(default)]
    pub ui_surfaces: Vec<String>,
    #[serde(default)]
    pub identity_requirements: Vec<String>,
    #[serde(default)]
    pub remote_access: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Trigger {
    #[serde(rename = "folder_changed")]
    FolderChanged { path: String },
    #[serde(rename = "schedule")]
    Schedule { cron: String },
    #[serde(rename = "event")]
    Event { name: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Permissions {
    #[serde(default)]
    pub fs_read: Vec<String>,
    #[serde(default)]
    pub fs_write: Vec<String>,
    #[serde(default)]
    pub network: bool,
    #[serde(default)]
    pub sensors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Budgets {
    #[serde(default)]
    pub solver_wake_per_hour: Option<u32>,
    #[serde(default)]
    pub max_ssd_writes_per_day_mb: Option<u32>,
    #[serde(default)]
    pub max_execution_time_ms: Option<u32>,
}

impl AgentManifest {
    pub fn parse_yaml(yaml: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }

    pub fn manifest_hash(&self) -> [u8; 32] {
        let s = serde_yaml::to_string(self).unwrap_or_default();
        let mut h = Sha256::new();
        h.update(s.as_bytes());
        h.finalize().into()
    }

    pub fn agent_uuid(&self) -> Uuid {
        // Stable-ish mapping for MVP: hash id into a UUID v5-like value.
        // Not cryptographically important; used for deterministic table keys.
        let mut h = Sha256::new();
        h.update(self.id.as_bytes());
        let digest: [u8; 32] = h.finalize().into();
        let mut b = [0u8; 16];
        b.copy_from_slice(&digest[..16]);
        Uuid::from_bytes(b)
    }
}


#[cfg(test)]
mod tests {
    use super::AgentManifest;

    const LEGACY_MANIFEST: &str = r#"id: goni.agent.legacy
version: 0.1.0
triggers:
  - type: schedule
    cron: "0 7 * * *"
permissions:
  fs_read: ["~/Documents"]
  fs_write: ["~/Documents/Out"]
  network: false
budgets:
  solver_wake_per_hour: 2
  max_ssd_writes_per_day_mb: 100
tools:
  - pdf_text_extract
"#;

    const NEW_MANIFEST: &str = r#"id: goni.agent.new
version: 0.2.0
triggers:
  - type: event
    name: inbox_digest
permissions:
  fs_read: ["~/Mail"]
  fs_write: ["~/Mail/Out"]
  network: true
budgets:
  solver_wake_per_hour: 4
  max_execution_time_ms: 60000
ui_surfaces:
  - dashboard_tile
  - inbox_sidebar
identity_requirements:
  - user_session
remote_access: true
tools:
  - report_writer
"#;

    #[test]
    fn parses_legacy_manifest_without_new_fields() {
        let manifest = AgentManifest::parse_yaml(LEGACY_MANIFEST).expect("legacy manifest should parse");
        assert!(manifest.ui_surfaces.is_empty());
        assert!(manifest.identity_requirements.is_empty());
        assert!(!manifest.remote_access);
    }

    #[test]
    fn parses_manifest_with_new_fields() {
        let manifest = AgentManifest::parse_yaml(NEW_MANIFEST).expect("new manifest should parse");
        assert_eq!(manifest.ui_surfaces, vec!["dashboard_tile", "inbox_sidebar"]);
        assert_eq!(manifest.identity_requirements, vec!["user_session"]);
        assert!(manifest.remote_access);
    }
}
