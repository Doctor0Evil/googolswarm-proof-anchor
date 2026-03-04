//! Batch Manager - Queue management and batch creation
//!
//! This module manages the queue of pending shards and creates
//! optimized batches for submission.

use crate::types::{BatchJob, AnchorConfig};
use crate::error::AnchorError;
use row_rpm_ledger::RowShard;
use sled::Db;
use uuid::Uuid;
use chrono::Utc;

/// Batch manager for queue operations
pub struct BatchManager {
    db: Db,
    queue_key: &'static [u8],
}

impl BatchManager {
    /// Create a new batch manager
    pub fn new(db_path: String) -> Result<Self, AnchorError> {
        let db = sled::open(db_path)?;
        Ok(Self {
            db,
            queue_key: b"pending_queue",
        })
    }

    /// Queue a shard for anchoring
    pub fn queue_shard(&self, shard: RowShard) -> Result<(), AnchorError> {
        let mut queue = self.get_queue()?;
        queue.push(shard);
        self.save_queue(&queue)?;
        Ok(())
    }

    /// Create a batch job from queue
    pub fn create_batch_job(&self, batch_size: usize) -> Result<BatchJob, AnchorError> {
        let mut queue = self.get_queue()?;
        
        let batch_shards: Vec<RowShard> = queue.drain(..batch_size.min(queue.len())).collect();
        
        self.save_queue(&queue)?;

        Ok(BatchJob {
            id: Uuid::new_v4().to_string(),
            shards: batch_shards,
            created_at: Utc::now().timestamp(),
            status: "pending".to_string(),
        })
    }

    /// Mark batch job as complete
    pub fn complete_batch_job(&self, job_id: &str) -> Result<(), AnchorError> {
        // In production, update job status in database
        log::info!("Batch job {} completed", job_id);
        Ok(())
    }

    /// Get pending queue count
    pub fn pending_count(&self) -> Result<usize, AnchorError> {
        let queue = self.get_queue()?;
        Ok(queue.len())
    }

    /// Get queue from database
    fn get_queue(&self) -> Result<Vec<RowShard>, AnchorError> {
        match self.db.get(self.queue_key)? {
            Some(data) => Ok(bincode::deserialize(&data)?),
            None => Ok(Vec::new()),
        }
    }

    /// Save queue to database
    fn save_queue(&self, queue: &[RowShard]) -> Result<(), AnchorError> {
        let data = bincode::serialize(queue)?;
        self.db.insert(self.queue_key, data)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_batch_manager_creation() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("batch.db").to_string_lossy().to_string();
        
        let manager = BatchManager::new(path);
        assert!(manager.is_ok());
    }
}
