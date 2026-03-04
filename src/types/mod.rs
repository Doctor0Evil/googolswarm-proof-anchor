//! Anchor Types - Data structures for anchoring operations
//!
//! This module defines core data structures used throughout
//! the anchoring system.

use serde::{Deserialize, Serialize};
use row_rpm_ledger::RowShard;

/// Anchor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnchorConfig {
    pub db_path: String,
    pub cache_path: String,
    pub googolswarm_endpoint: String,
    pub organichain_endpoint: String,
    pub batch_size: usize,
    pub retry_config: RetryConfig,
}

/// Retry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
}

impl Default for AnchorConfig {
    fn default() -> Self {
        Self {
            db_path: "/var/lib/aln/anchor.db".to_string(),
            cache_path: "/var/lib/aln/anchor_cache".to_string(),
            googolswarm_endpoint: "https://api.googolswarm.net".to_string(),
            organichain_endpoint: "https://api.organichain.io".to_string(),
            batch_size: 1000,
            retry_config: RetryConfig::default(),
        }
    }
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 5,
            initial_delay_ms: 1000,
            max_delay_ms: 60000,
        }
    }
}

/// Anchor proof receipt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnchorProof {
    pub proof_id: String,
    pub shard_id: String,
    pub ledger_type: String,
    pub transaction_id: String,
    pub block_height: u64,
    pub timestamp: i64,
    pub merkle_proof: Vec<u8>,
    pub hex_stamp: String,
}

/// Batch job for submission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchJob {
    pub id: String,
    pub shards: Vec<RowShard>,
    pub created_at: i64,
    pub status: String,
}

/// Anchor status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnchorStatus {
    pub pending_count: usize,
    pub anchored_count: usize,
    pub failed_count: usize,
    pub last_sync: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        let config = AnchorConfig::default();
        assert_eq!(config.batch_size, 1000);
    }
}
