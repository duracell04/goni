use std::collections::VecDeque;
use std::sync::Arc;

use async_trait::async_trait;
use goni_types::{GoniBatch, TaskClass, BatchMeta};
use tokio::sync::Mutex;
use arrow::record_batch::RecordBatch;
use arrow::datatypes::Schema;

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
}

fn idx_for(class: TaskClass) -> usize {
    match class {
        TaskClass::Interactive => 0,
        TaskClass::Background => 1,
        TaskClass::Maintenance => 2,
    }
}

#[async_trait]
impl Scheduler for InMemoryScheduler {
    async fn submit(&self, batch: GoniBatch) -> Result<(), SchedError> {
        let mut inner = self.inner.lock().await;
        let idx = idx_for(batch.meta.class);
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

/// QoS scheduler with simple admission control.
pub struct QoSScheduler {
    inner: Mutex<QosInner>,
}

struct QosInner {
    queues: [VecDeque<Arc<GoniBatch>>; 3],
    max_wip: [usize; 3],
}

impl QoSScheduler {
    pub fn new(max_background: usize, max_maintenance: usize) -> Self {
        Self {
            inner: Mutex::new(QosInner {
                queues: [VecDeque::new(), VecDeque::new(), VecDeque::new()],
                max_wip: [usize::MAX, max_background, max_maintenance],
            }),
        }
    }
}

#[async_trait]
impl Scheduler for QoSScheduler {
    async fn submit(&self, batch: GoniBatch) -> Result<(), SchedError> {
        let mut inner = self.inner.lock().await;
        let idx = idx_for(batch.meta.class);
        if inner.queues[idx].len() >= inner.max_wip[idx] {
            return Err(SchedError {
                message: "wip_limit_reached".into(),
            });
        }
        inner.queues[idx].push_back(Arc::new(batch));
        Ok(())
    }

    async fn next(&self) -> Option<GoniBatch> {
        let mut inner = self.inner.lock().await;
        let order = [TaskClass::Interactive, TaskClass::Background, TaskClass::Maintenance];
        for class in order {
            let idx = idx_for(class);
            if let Some(batch) = inner.queues[idx].pop_front() {
                return Some(Arc::try_unwrap(batch).unwrap_or_else(|a| (*a).clone()));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn dummy_batch(class: TaskClass) -> GoniBatch {
        let schema = Arc::new(Schema::empty());
        let batch = RecordBatch::new_empty(schema);
        GoniBatch {
            data: Arc::new(batch),
            meta: BatchMeta {
                id: uuid::Uuid::new_v4(),
                class,
                arrival_ts: std::time::Instant::now(),
                est_tokens: 1,
            },
        }
    }

    #[tokio::test]
    async fn interactive_preferred_over_background() {
        let sched = InMemoryScheduler::new();
        sched.submit(dummy_batch(TaskClass::Background)).await.unwrap();
        sched.submit(dummy_batch(TaskClass::Interactive)).await.unwrap();

        let first = sched.next().await.expect("should pop a batch");
        assert_eq!(first.meta.class, TaskClass::Interactive);
    }

    #[tokio::test]
    async fn background_limit_enforced() {
        let sched = QoSScheduler::new(0, 0);
        let res = sched.submit(dummy_batch(TaskClass::Background)).await;
        assert!(res.is_err());
    }
}

