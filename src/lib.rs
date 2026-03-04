//! Googolswarm Proof Anchor - Immutable proof submission with offline support
//!
//! This crate provides batching, submission, and verification of ROW/RPM shards
//! to Googolswarm/Organichain ledgers with full offline support.
//!
//! # Architecture
//!
//! ```text
//! ROW/RPM Ledger → BatchManager → Googolswarm API → Proof Verification → Local Cache
//! ```
//!
//! # Example
//!
//! ```rust
//! use googolswarm_proof_anchor::{AnchorManager, AnchorConfig};
//!
//! let config = AnchorConfig::default();
//! let mut manager = AnchorManager::new(config)?;
//!
//! // Queue shards for anchoring
//! manager.queue_shard(shard)?;
//!
//! // Submit batch when online
//! manager.submit_batch(100).await?;
//!
//! // Verify proof offline
//! let valid = manager.verify_proof(&proof_id)?;
//! ```

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![allow(clippy::module_name_repetitions)]

pub mod anchoring;
pub mod verification;
pub mod batch;
pub mod retry;
pub mod cache;
pub mod error;
pub mod types;
pub mod hex_stamp;

/// Crate version
pub const VERSION: &str = "1.0.0";

/// Hex-stamp attestation for this release
pub const HEX_STAMP: &str = "0xcf8f4e7d6c3b9a1f0e5d4c3b2a1f0e9d8c7b6a59f8e7d6c5b4a3928170f6e5d4";

/// Ledger reference for this release
pub const LEDGER_REF: &str = "row:googolswarm-proof-anchor:v1.0.0:2026-03-04";

/// Re-export commonly used types
pub use anchoring::AnchorManager;
pub use types::{AnchorConfig, AnchorProof, BatchJob};
pub use error::AnchorError;

/// Submit a batch of shards for anchoring
///
/// # Arguments
///
/// * `shards` - List of ROW/RPM shards to anchor
/// * `batch_size` - Maximum batch size
///
/// # Returns
///
/// * `Vec<AnchorProof>` - Proofs for each anchored shard
pub async fn submit_batch(
    shards: Vec<row_rpm_ledger::RowShard>,
    batch_size: usize,
) -> Result<Vec<AnchorProof>, AnchorError> {
    let config = AnchorConfig::default();
    let mut manager = AnchorManager::new(config)?;
    
    for shard in shards {
        manager.queue_shard(shard)?;
    }
    
    manager.submit_batch(batch_size).await
}

/// Verify an anchor proof offline
///
/// # Arguments
///
/// * `proof_id` - Proof identifier to verify
///
/// # Returns
///
/// * `bool` - True if valid, false otherwise
pub fn verify_proof_offline(proof_id: &str) -> Result<bool, AnchorError> {
    let config = AnchorConfig::default();
    let manager = AnchorManager::new(config)?;
    manager.verify_proof_from_cache(proof_id)
}

/// Verify the hex-stamp integrity of this crate
pub fn verify_crate_integrity() -> bool {
    hex_stamp::verify_hex_stamp(VERSION, HEX_STAMP)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crate_version() {
        assert_eq!(VERSION, "1.0.0");
    }

    #[test]
    fn test_hex_stamp_format() {
        assert!(HEX_STAMP.starts_with("0x"));
        assert_eq!(HEX_STAMP.len(), 66);
    }

    #[test]
    fn test_manager_creation() {
        let config = AnchorConfig::default();
        let manager = AnchorManager::new(config);
        assert!(manager.is_ok());
    }
}
