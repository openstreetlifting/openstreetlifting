pub mod policy;

use axum::body::Bytes;
use std::future::Future;
use tokio::{sync::Mutex, time::Instant};
use tracing::Instrument;

use policy::CachePolicy;

pub struct AppCaches {
    pub ris_distribution: Cache<Bytes>,
}

impl AppCaches {
    #[must_use]
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
            metrics::counter!("osl_cache_requests_total", "cache" => self.policy.name, "result" => "bypass").increment(1);
            return load().await;
        }

        let mut entry = self.entry.lock().await;
        if let Some(cached) = entry.as_ref()
            && cached.expires_at > Instant::now()
        {
            tracing::debug!(
                osl.cache.name = self.policy.name,
                "Cached dataset retrieved"
            );
            metrics::counter!("osl_cache_requests_total", "cache" => self.policy.name, "result" => "hit").increment(1);
            return Ok(cached.value.clone());
        }

        let start = Instant::now();
        metrics::counter!("osl_cache_requests_total", "cache" => self.policy.name, "result" => "miss").increment(1);
        let result = load()
            .instrument(tracing::info_span!(
                "cache.refresh",
                osl.cache.name = self.policy.name
            ))
            .await;
        metrics::histogram!("osl_cache_refresh_duration_seconds", "cache" => self.policy.name)
            .record(start.elapsed().as_secs_f64());
        let value = match result {
            Ok(value) => value,
            Err(error) => {
                tracing::debug!(
                    osl.cache.name = self.policy.name,
                    "Dataset cache refresh failed"
                );
                metrics::counter!("osl_cache_refreshes_total", "cache" => self.policy.name, "result" => "error").increment(1);
                // Neither errors nor expired snapshots are served as successful results.
                return Err(error);
            }
        };
        metrics::counter!("osl_cache_refreshes_total", "cache" => self.policy.name, "result" => "success").increment(1);
        *entry = Some(Entry {
            value: value.clone(),
            expires_at: Instant::now() + self.policy.ttl,
        });
        drop(entry);
        tracing::debug!(
            osl.cache.name = self.policy.name,
            elapsed_ms = u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX),
            "Cache refreshed"
        );
        Ok(value)
    }
}

#[cfg(test)]
mod tests;
