use super::*;
use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use tokio::sync::Notify;

const POLICY: CachePolicy = CachePolicy {
    name: "test",
    ttl: Duration::from_secs(60),
};

#[tokio::test]
async fn disabled_cache_always_loads_without_storing_a_snapshot() {
    let cache = Cache::new(false, POLICY);
    assert_eq!(
        cache.get_or_try_init(|| async { Ok::<_, ()>(1) }).await,
        Ok(1)
    );
    assert_eq!(
        cache.get_or_try_init(|| async { Ok::<_, ()>(2) }).await,
        Ok(2)
    );
    assert!(cache.entry.lock().await.is_none());
}

#[tokio::test(start_paused = true)]
async fn hits_reuse_the_snapshot_and_expiry_loads_updated_data() {
    let cache = Cache::new(true, POLICY);
    assert_eq!(
        cache.get_or_try_init(|| async { Ok::<_, ()>(1) }).await,
        Ok(1)
    );
    tokio::time::advance(Duration::from_secs(59)).await;
    assert_eq!(
        cache.get_or_try_init(|| async { Ok::<_, ()>(2) }).await,
        Ok(1)
    );
    tokio::time::advance(Duration::from_secs(1)).await;
    assert_eq!(
        cache.get_or_try_init(|| async { Ok::<_, ()>(2) }).await,
        Ok(2)
    );
}

#[tokio::test(start_paused = true)]
async fn ttl_starts_after_a_successful_load_not_before_it() {
    let cache = Cache::new(true, POLICY);
    cache
        .get_or_try_init(|| async {
            tokio::time::sleep(Duration::from_secs(90)).await;
            Ok::<_, ()>(1)
        })
        .await
        .unwrap();
    assert_eq!(
        cache.get_or_try_init(|| async { Ok::<_, ()>(2) }).await,
        Ok(1)
    );
}

#[tokio::test(start_paused = true)]
async fn concurrent_requests_share_one_successful_refresh_on_cold_start_and_expiry() {
    let cache = Arc::new(Cache::new(true, POLICY));
    let calls = Arc::new(AtomicUsize::new(0));
    for round in 1..=2 {
        let mut tasks = tokio::task::JoinSet::new();
        for _ in 0..20 {
            let cache = Arc::clone(&cache);
            let calls = Arc::clone(&calls);
            tasks.spawn(async move {
                cache
                    .get_or_try_init(|| async {
                        let count = calls.fetch_add(1, Ordering::SeqCst) + 1;
                        tokio::time::sleep(Duration::from_secs(1)).await;
                        Ok::<_, ()>(count)
                    })
                    .await
            });
        }
        while let Some(result) = tasks.join_next().await {
            assert_eq!(result.unwrap(), Ok(round));
        }
        assert_eq!(calls.load(Ordering::SeqCst), round);
        tokio::time::advance(POLICY.ttl).await;
    }
}

#[tokio::test(start_paused = true)]
async fn errors_are_not_cached_and_expired_values_are_not_returned_on_failure() {
    let cache = Cache::new(true, POLICY);
    assert_eq!(
        cache
            .get_or_try_init(|| async { Err::<i32, _>("offline") })
            .await,
        Err("offline")
    );
    assert!(cache.entry.lock().await.is_none());
    assert_eq!(
        cache.get_or_try_init(|| async { Ok::<_, &str>(1) }).await,
        Ok(1)
    );
    tokio::time::advance(POLICY.ttl).await;
    assert_eq!(
        cache
            .get_or_try_init(|| async { Err::<i32, _>("offline") })
            .await,
        Err("offline")
    );
    assert_eq!(
        cache.get_or_try_init(|| async { Ok::<_, &str>(2) }).await,
        Ok(2)
    );
}

#[tokio::test]
async fn a_cancelled_refresh_does_not_leave_the_cache_locked() {
    let cache = Arc::new(Cache::new(true, POLICY));
    let started = Arc::new(Notify::new());
    let task = {
        let cache = Arc::clone(&cache);
        let started = Arc::clone(&started);
        tokio::spawn(async move {
            cache
                .get_or_try_init(|| async {
                    started.notify_one();
                    std::future::pending::<Result<i32, ()>>().await
                })
                .await
        })
    };
    started.notified().await;
    task.abort();
    assert!(task.await.unwrap_err().is_cancelled());
    let result = tokio::time::timeout(
        Duration::from_secs(1),
        cache.get_or_try_init(|| async { Ok::<_, ()>(2) }),
    )
    .await;
    assert_eq!(result.unwrap(), Ok(2));
}

#[tokio::test]
async fn empty_successful_datasets_are_cached() {
    let cache = Cache::new(true, POLICY);
    let empty: Vec<u8> = Vec::new();
    assert_eq!(
        cache
            .get_or_try_init(|| async { Ok::<_, ()>(empty.clone()) })
            .await,
        Ok(empty.clone())
    );
    assert_eq!(
        cache
            .get_or_try_init(|| async { Err::<Vec<u8>, _>(()) })
            .await,
        Ok(empty)
    );
}
