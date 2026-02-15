//! Async task scheduler — resource-aware

use std::sync::atomic::{AtomicU64, Ordering};
use tokio::sync::Semaphore;

/// Resource-aware scheduler. Limits concurrent tasks.
pub struct TaskScheduler {
    semaphore: Semaphore,
    active: AtomicU64,
}

impl TaskScheduler {
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            semaphore: Semaphore::new(max_concurrent),
            active: AtomicU64::new(0),
        }
    }

    pub async fn acquire(&self) -> tokio::sync::SemaphorePermit<'_> {
        self.semaphore.acquire().await.expect("semaphore closed")
    }

    pub fn active_count(&self) -> u64 {
        self.active.load(Ordering::Relaxed)
    }
}
