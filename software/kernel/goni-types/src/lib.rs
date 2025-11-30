use std::sync::Arc;
use std::time::Instant;

use arrow::record_batch::RecordBatch;
use uuid::Uuid;

/// Classification for tasks in the scheduler.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum TaskClass {
    Interactive,
    Background,
    Maintenance,
}

/// Metadata about a batch flowing through the kernel.
#[derive(Clone, Debug)]
pub struct BatchMeta {
    pub id: Uuid,
    pub class: TaskClass,
    pub arrival_ts: Instant,
    /// Estimated total tokens the request will consume.
    pub est_tokens: usize,
}

/// Atomic unit in the data/scheduler/context planes.
#[derive(Clone)]
pub struct GoniBatch {
    pub data: Arc<RecordBatch>,
    pub meta: BatchMeta,
}

/// Identifier for a page of KV cache in device memory.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct KvPageId(pub u64);

/// Context selection result: which chunks to use and total tokens.
#[derive(Clone, Debug)]
pub struct ContextSelection {
    pub indices: Vec<u32>,   // indices into candidate set
    pub total_tokens: usize, // total tokens across selected chunks
}

/// Tier / model choice for the LLM.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ModelTier {
    LocalSmall,
    LocalLarge,
    RemoteHeavy,
}

/// Single LLM request handed to the inference engine.
#[derive(Clone, Debug)]
pub struct LlmRequest {
    pub prompt: String,
    pub context: ContextSelection,
    pub model_tier: ModelTier,
    pub max_tokens: usize,
}
