//! Anchor Daemon - Background service for continuous anchoring

use googolswarm_proof_anchor::{AnchorManager, AnchorConfig};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let config = AnchorConfig::default();
    let mut manager = AnchorManager::new(config)?;

    log::info!("Anchor daemon started");

    loop {
        // Check pending queue
        let pending = manager.pending_count()?;
        
        if pending > 0 {
            log::info!("Processing {} pending shards", pending);
            
            // Submit batch
            match manager.submit_batch(1000).await {
                Ok(proofs) => {
                    log::info!("Successfully anchored {} proofs", proofs.len());
                }
                Err(e) => {
                    log::warn!("Anchoring failed: {}", e);
                    // Retry with backoff
                    tokio::time::sleep(Duration::from_secs(60)).await;
                    continue;
                }
            }
        }

        // Wait before next check
        tokio::time::sleep(Duration::from_secs(30)).await;
    }
}
