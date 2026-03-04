//! Retry Logic - Exponential backoff with jitter
//!
//! This module implements resilient retry logic for failed
//! anchoring submissions.

use serde::{Deserialize, Serialize};
use std::time::Duration;
use rand::Rng;

/// Retry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
    pub multiplier: f64,
    pub jitter: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 5,
            initial_delay_ms: 1000,
            max_delay_ms: 60000,
            multiplier: 2.0,
            jitter: true,
        }
    }
}

/// Retry executor
pub struct RetryExecutor {
    config: RetryConfig,
}

impl RetryExecutor {
    /// Create a new retry executor
    pub fn new(config: RetryConfig) -> Self {
        Self { config }
    }

    /// Calculate delay for current attempt
    pub fn calculate_delay(&self, attempt: u32) -> Duration {
        let mut delay = self.config.initial_delay_ms as f64 
            * self.config.multiplier.powi(attempt as i32);
        
        delay = delay.min(self.config.max_delay_ms as f64);

        if self.config.jitter {
            let mut rng = rand::thread_rng();
            let jitter = rng.gen_range(0.8..1.2);
            delay *= jitter;
        }

        Duration::from_millis(delay as u64)
    }

    /// Execute function with retry
    pub async fn execute_with_retry<F, T, E>(
        &self,
        mut func: F,
    ) -> Result<T, E>
    where
        F: FnMut() -> futures::future::BoxFuture<'static, Result<T, E>>,
        E: std::fmt::Debug,
    {
        let mut attempt = 0;
        
        loop {
            match func().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    attempt += 1;
                    if attempt >= self.config.max_retries {
                        return Err(e);
                    }
                    
                    let delay = self.calculate_delay(attempt);
                    log::warn!("Retry attempt {} failed, waiting {:?}", attempt, delay);
                    tokio::time::sleep(delay).await;
                }
            }
        }
    }
}

impl Default for RetryExecutor {
    fn default() -> Self {
        Self::new(RetryConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delay_calculation() {
        let executor = RetryExecutor::new(RetryConfig::default());
        
        let delay0 = executor.calculate_delay(0);
        let delay1 = executor.calculate_delay(1);
        
        assert!(delay1 >= delay0);
    }
}
