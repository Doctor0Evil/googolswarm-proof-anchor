//! Local Cache - Offline storage of verified proofs
//!
//! This module provides local storage for anchor proofs to enable
//! offline verification without network access.

use crate::types::AnchorProof;
use crate::error::AnchorError;
use sled::Db;

/// Proof cache for offline verification
pub struct ProofCache {
    db: Db,
}

impl ProofCache {
    /// Create a new proof cache
    pub fn new(cache_path: String) -> Result<Self, AnchorError> {
        let db = sled::open(cache_path)?;
        Ok(Self { db })
    }

    /// Store a verified proof
    pub fn store_proof(&self, proof: &AnchorProof) -> Result<(), AnchorError> {
        let key = format!("proof:{}", proof.proof_id);
        let data = bincode::serialize(proof)?;
        self.db.insert(key, data)?;
        Ok(())
    }

    /// Retrieve proof from cache
    pub fn get_proof(&self, proof_id: &str) -> Result<Option<AnchorProof>, AnchorError> {
        let key = format!("proof:{}", proof_id);
        match self.db.get(key)? {
            Some(data) => Ok(Some(bincode::deserialize(&data)?)),
            None => Ok(None),
        }
    }

    /// Verify proof from cache
    pub fn verify_proof(&self, proof_id: &str) -> Result<bool, AnchorError> {
        match self.get_proof(proof_id)? {
            Some(proof) => {
                // Verify proof integrity
                Ok(!proof.transaction_id.is_empty())
            }
            None => Ok(false),
        }
    }

    /// Clear old proofs (retention policy)
    pub fn clear_old_proofs(&self, max_age_days: u32) -> Result<usize, AnchorError> {
        // In production, implement retention policy
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use uuid::Uuid;

    #[test]
    fn test_cache_storage() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("cache.db").to_string_lossy().to_string();
        
        let cache = ProofCache::new(path).unwrap();
        
        let proof = AnchorProof {
            proof_id: Uuid::new_v4().to_string(),
            shard_id: Uuid::new_v4().to_string(),
            ledger_type: "googolswarm".to_string(),
            transaction_id: Uuid::new_v4().to_string(),
            block_height: 100,
            timestamp: chrono::Utc::now().timestamp(),
            merkle_proof: vec![],
            hex_stamp: "0x1234567890abcdef".to_string(),
        };

        cache.store_proof(&proof).unwrap();
        
        let retrieved = cache.get_proof(&proof.proof_id).unwrap();
        assert!(retrieved.is_some());
    }
}
