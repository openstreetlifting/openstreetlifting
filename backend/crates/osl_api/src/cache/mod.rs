pub mod policy;

use axum::body::Bytes;
use std::future::Future;
use tokio::{sync::Mutex, time::Instant};

use policy::CachePolicy;

/// One bounded snapshot per dataset. Add typed slots here as new policies are needed.
pub struct AppCaches {
    pub ris_distribution: Cache<Bytes>,
}

impl AppCaches {
    pub fn new(enabled: bool) -> Self {
        Self {
            ris_distribution: Cache::new(enabled, policy::RIS_DISTRIBUTION),
        }
    }
}

struct Entry<T> {
    value: T,
    expires_at: Instant,
}

/// A process-local, lazy TTL cache. Use cheaply cloned values such as Bytes or Arc<T>.
/// The lock serializes refreshes; cancellation releases it without publishing partial data.
pub struct Cache<T> {
    enabled: bool,
    policy: CachePolicy,
    entry: Mutex<Option<Entry<T>>>,
}

impl<T: Clone> Cache<T> {
    pub fn new(enabled: bool, policy: CachePolicy) -> Self {
        Self {
            enabled,
            policy,
            entry: Mutex::new(None),
        }
    }

    pub async fn get_or_try_init<F, Fut, E>(&self, load: F) -> Result<T, E>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<T, E>>,
    {
        if !self.enabled {
            return load().await;
        }

        let mut entry = self.entry.lock().await;
        if let Some(cached) = entry.as_ref()
            && cached.expires_at > Instant::now()
        {
            tracing::debug!(cache = self.policy.name, "Cache hit");
            return Ok(cached.value.clone());
        }

        let start = Instant::now();
        let value = match load().await {
            Ok(value) => value,
            Err(error) => {
                tracing::warn!(cache = self.policy.name, "Cache refresh failed");
                // Neither errors nor expired snapshots are served as successful results.
                return Err(error);
            }
        };
        *entry = Some(Entry {
            value: value.clone(),
            expires_at: Instant::now() + self.policy.ttl,
        });
        tracing::debug!(
            cache = self.policy.name,
            elapsed_ms = start.elapsed().as_millis() as u64,
            "Cache refreshed"
        );
        Ok(value)
    }
}

#[cfg(test)]
mod tests;
