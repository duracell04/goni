use std::collections::VecDeque;
use std::sync::Arc;

use async_trait::async_trait;
use goni_types::{GoniBatch, TaskClass};
use tokio::sync::Mutex;

/// Core scheduling interface.
#[async_trait]
pub trait Scheduler: Send + Sync {
    async fn submit(&self, batch: GoniBatch) -> Result<(), SchedError>;
    async fn next(&self) -> Option<GoniBatch>;
}

#[derive(Debug)]
pub struct SchedError {
    pub message: String,
}

/// Simple in-memory MaxWeight-ish scheduler.
/// For now we assume same service rate across classes; later you plug in EMA-based µ.
pub struct InMemoryScheduler {
    inner: Mutex<Inner>,
}

struct Inner {
    queues: [VecDeque<Arc<GoniBatch>>; 3],
    weights: [f64; 3], // w_int, w_bg, w_maint
}

impl InMemoryScheduler {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(Inner {
                queues: [
                    VecDeque::new(),
                    VecDeque::new(),
                    VecDeque::new(),
                ],
                weights: [1000.0, 10.0, 1.0],
            }),
        }
    }

    fn idx_for(class: TaskClass) -> usize {
        match class {
            TaskClass::Interactive => 0,
            TaskClass::Background => 1,
            TaskClass::Maintenance => 2,
        }
    }
}

#[async_trait]
impl Scheduler for InMemoryScheduler {
    async fn submit(&self, batch: GoniBatch) -> Result<(), SchedError> {
        let mut inner = self.inner.lock().await;
        let idx = Inner::idx_for(batch.meta.class);
        inner.queues[idx].push_back(Arc::new(batch));
        Ok(())
    }

    async fn next(&self) -> Option<GoniBatch> {
        let mut inner = self.inner.lock().await;

        // MaxWeight simplified: pick queue with largest w_i * Q_i
        let mut best_idx: Option<usize> = None;
        let mut best_score = f64::MIN;

        for (i, q) in inner.queues.iter().enumerate() {
            let q_len = q.len() as f64;
            if q_len == 0.0 {
                continue;
            }
            let score = inner.weights[i] * q_len;
            if score > best_score {
                best_score = score;
                best_idx = Some(i);
            }
        }

        if let Some(idx) = best_idx {
            inner.queues[idx]
                .pop_front()
                .map(|arc| Arc::try_unwrap(arc).unwrap_or_else(|a| (*a).clone()))
        } else {
            None
        }
    }
}
