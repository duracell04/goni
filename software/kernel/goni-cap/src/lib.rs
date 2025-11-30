use async_trait::async_trait;
use cap_std::fs::Dir;
use serde_json::Value;

/// Filesystem capability: root directory handle.
pub struct FsCap {
    pub root: Dir,
}

/// Network capability: zero or restricted domains.
pub enum NetCap {
    NoNet,
    AllowDomains(Vec<String>),
}

/// Budget capabilities for tokens and energy (simplified).
pub struct TokenCap {
    pub remaining: u64,
}

pub struct EnergyCap {
    pub remaining_mj: f64,
}

#[derive(Debug)]
pub struct ToolError {
    pub message: String,
}

#[async_trait]
pub trait Tool: Send + Sync {
    async fn invoke(
        &self,
        fs: &FsCap,
        net: &NetCap,
        tokens: &mut TokenCap,
        energy: &mut EnergyCap,
        input: Value,
    ) -> Result<Value, ToolError>;
}
