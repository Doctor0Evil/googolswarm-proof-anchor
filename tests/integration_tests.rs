//! Googolswarm Proof Anchor Integration Tests

use googolswarm_proof_anchor::{AnchorManager, AnchorConfig, RowShard};
use tempfile::tempdir;

#[tokio::test]
async fn test_full_anchor_lifecycle() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("anchor.db").to_string_lossy().to_string();
    let cache_path = dir.path().join("cache").to_string_lossy().to_string();

    let config = AnchorConfig {
        db_path,
        cache_path,
        ..Default::default()
    };

    let mut manager = AnchorManager::new(config).unwrap();

    // Create test shard
    let shard = create_test_shard();

    // Queue shard
    manager.queue_shard(shard).unwrap();

    // Check pending count
    let pending = manager.pending_count().unwrap();
    assert_eq!(pending, 1);

    // Submit batch
    let proofs = manager.submit_batch(100).await.unwrap();
    assert!(!proofs.is_empty());

    // Verify proof from cache
    let valid = manager.verify_proof_from_cache(&proofs[0].proof_id).unwrap();
    assert!(valid);
}

#[tokio::test]
async fn test_offline_queueing() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("anchor.db").to_string_lossy().to_string();
    let cache_path = dir.path().join("cache").to_string_lossy().to_string();

    let config = AnchorConfig {
        db_path,
        cache_path,
        ..Default::default()
    };

    let mut manager = AnchorManager::new(config).unwrap();

    // Queue multiple shards
    for i in 0..10 {
        let shard = create_test_shard();
        manager.queue_shard(shard).unwrap();
    }

    // Check pending count
    let pending = manager.pending_count().unwrap();
    assert_eq!(pending, 10);
}

fn create_test_shard() -> RowShard {
    // In production, create valid RowShard
    // For now, return placeholder
    use row_rpm_ledger::RowShard;
    use row_rpm_ledger::shard::{ResourceRequest, ResourceGrant, EcoVector};
    
    RowShard::new(
        "session-test".to_string(),
        "test".to_string(),
        ResourceRequest {
            cpu_cores: 1,
            memory_mb: 1024,
            network_bandwidth_mbps: 10.0,
            storage_gb: 10,
            swarm_nodes: 1,
            duration_seconds: 60,
        },
        ResourceGrant {
            cpu_cores: 1,
            memory_mb: 1024,
            network_bandwidth_mbps: 10.0,
            storage_gb: 10,
            swarm_nodes: 1,
            duration_seconds: 60,
            quota_remaining_pct: 1.0,
        },
        EcoVector {
            gco2_per_joule: 0.001,
            eco_impact_score: 0.5,
            energy_autonomy_pct: 0.8,
            eco_floor_minimum: 0.3,
        },
        "Normal".to_string(),
        "bostrom1test".to_string(),
        "bostrom1test".to_string(),
        "cyb:test".to_string(),
    )
}
