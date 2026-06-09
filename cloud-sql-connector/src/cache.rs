//! A single-flight, expiry-aware cache holding one refreshable value.

use core::future::Future;
use std::sync::Arc;
use std::time::SystemTime;

use tokio::sync::Mutex;

/// Produces the cache's value together with the time it should be refreshed.
///
/// The cache owns its fetcher, so [`get`](RefreshCache::get) and
/// [`refresh`](RefreshCache::refresh) need no per-call closure. The returned
/// future borrows the fetcher, but it is awaited *inside* the cache and never
/// escapes across a caller's `await`.
pub(crate) trait Fetch {
    /// The cached value.
    type Value;
    /// The error a fetch can fail with.
    type Error;

    /// Fetch a fresh value and the time at which it becomes stale.
    fn fetch(&self) -> impl Future<Output = Result<(Self::Value, SystemTime), Self::Error>> + Send;
}

/// A cache holding a single value, refetched via its [`Fetch`]er when the value
/// is absent or has passed its refresh deadline.
///
/// Refetches are single-flight: the lock is held across the fetch, so when many
/// callers find the value stale at once, exactly one fetches and the rest reuse
/// its result.
pub(crate) struct RefreshCache<F: Fetch> {
    slot: Mutex<Option<Entry<F::Value>>>,
    fetcher: F,
}

impl<F: Fetch> core::fmt::Debug for RefreshCache<F> {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("RefreshCache")
            .finish_non_exhaustive()
    }
}

/// One cached fill: the value and the time at which it must be refreshed.
struct Entry<T> {
    value: Arc<T>,
    refresh_after: SystemTime,
}

impl<T> Entry<T> {
    /// Whether `now` has reached this fill's refresh deadline.
    fn is_stale(&self, now: SystemTime) -> bool {
        now >= self.refresh_after
    }
}

impl<F: Fetch> RefreshCache<F> {
    /// Create an empty cache backed by `fetcher`.
    pub(crate) fn new(fetcher: F) -> Self {
        Self {
            slot: Mutex::new(None),
            fetcher,
        }
    }

    /// Return a usable value, fetching a new one if the cache is empty or stale.
    pub(crate) async fn get(&self, now: SystemTime) -> Result<Arc<F::Value>, F::Error> {
        let mut guard = self.slot.lock().await;

        if let Some(entry) = guard.as_ref()
            && !entry.is_stale(now)
        {
            return Ok(Arc::clone(&entry.value));
        }

        self.refill(&mut guard).await
    }

    /// Fetch a new value after the value `failed` let the caller down, unless
    /// the cache already holds a newer value that is still fresh.
    ///
    /// `failed` is the value the caller obtained from [`get`](Self::get) and
    /// could not use (for example, the connection it described failed). If the
    /// cache still holds that same value, or holds a replacement that is itself
    /// stale, a new value is fetched; otherwise the newer value is reused. The
    /// staleness recheck matters because the caller has no bound on how long it
    /// blocked on the lock: a replacement installed by another caller may have
    /// aged past its own deadline in the meantime.
    pub(crate) async fn refresh(
        &self,
        now: SystemTime,
        failed: &Arc<F::Value>,
    ) -> Result<Arc<F::Value>, F::Error> {
        let mut guard = self.slot.lock().await;

        if let Some(entry) = guard.as_ref()
            && !Arc::ptr_eq(&entry.value, failed)
            && !entry.is_stale(now)
        {
            return Ok(Arc::clone(&entry.value));
        }

        self.refill(&mut guard).await
    }

    /// Fetch a value through the cache's fetcher and store it under the held
    /// lock, returning the stored `Arc`.
    async fn refill(&self, guard: &mut Option<Entry<F::Value>>) -> Result<Arc<F::Value>, F::Error> {
        let (value, refresh_after) = self.fetcher.fetch().await?;
        let value = Arc::new(value);
        *guard = Some(Entry {
            value: Arc::clone(&value),
            refresh_after,
        });
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;

    use tokio::sync::{Barrier, Notify};

    /// Error type for the test fetcher.
    #[derive(Clone, Debug, PartialEq)]
    struct FetchError;

    /// One hour — the gap between a fill and its refresh deadline in the fresh
    /// cases below.
    const HOUR: Duration = Duration::from_secs(3600);

    /// A fetcher that returns a preset sequence of responses (one per call),
    /// counts calls, and optionally blocks the fetch at/after a given call index
    /// so a test can hold the single-flight window open.
    struct TestFetcher {
        calls: Arc<AtomicUsize>,
        responses: Vec<Result<(u32, SystemTime), FetchError>>,
        gate: Arc<Notify>,
        gate_from: usize,
    }

    impl TestFetcher {
        /// Build a fetcher with the given response sequence; returns it and a
        /// shared handle to its call counter.
        fn new(responses: Vec<Result<(u32, SystemTime), FetchError>>) -> (Self, Arc<AtomicUsize>) {
            let calls = Arc::new(AtomicUsize::new(0));
            let fetcher = Self {
                calls: Arc::clone(&calls),
                responses,
                gate: Arc::new(Notify::new()),
                gate_from: usize::MAX,
            };
            (fetcher, calls)
        }
    }

    impl Fetch for TestFetcher {
        type Value = u32;
        type Error = FetchError;

        async fn fetch(&self) -> Result<(u32, SystemTime), FetchError> {
            let n = self.calls.fetch_add(1, Ordering::SeqCst);
            if n >= self.gate_from {
                self.gate.notified().await;
            }
            self.responses[n].clone()
        }
    }

    #[tokio::test]
    async fn get_on_empty_fetches() {
        let now = SystemTime::UNIX_EPOCH;
        let (fetcher, calls) = TestFetcher::new(vec![Ok((7, now + HOUR))]);
        let cache = RefreshCache::new(fetcher);

        let value = cache.get(now).await.unwrap();

        assert_eq!(*value, 7);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn get_when_fresh_reuses_without_fetching() {
        let now = SystemTime::UNIX_EPOCH;
        let (fetcher, calls) = TestFetcher::new(vec![Ok((7, now + HOUR)), Ok((99, now + HOUR))]);
        let cache = RefreshCache::new(fetcher);

        let first = cache.get(now).await.unwrap();
        // Still before the deadline: the second response must never be used.
        let second = cache.get(now).await.unwrap();

        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert!(Arc::ptr_eq(&first, &second));
        assert_eq!(*second, 7);
    }

    #[tokio::test]
    async fn get_when_stale_refetches() {
        let now = SystemTime::UNIX_EPOCH;
        let deadline = now + HOUR;
        let (fetcher, calls) = TestFetcher::new(vec![Ok((7, deadline)), Ok((8, deadline + HOUR))]);
        let cache = RefreshCache::new(fetcher);

        let first = cache.get(now).await.unwrap();
        // At the deadline the entry is stale (`>=`), so this refetches.
        let second = cache.get(deadline).await.unwrap();

        assert_eq!(calls.load(Ordering::SeqCst), 2);
        assert!(!Arc::ptr_eq(&first, &second));
        assert_eq!(*second, 8);
    }

    #[tokio::test]
    async fn refresh_reuses_newer_fresh_entry() {
        let now = SystemTime::UNIX_EPOCH;
        let (fetcher, calls) = TestFetcher::new(vec![
            Ok((1, now + HOUR)),
            Ok((2, now + HOUR)),
            Ok((3, now + HOUR)),
        ]);
        let cache = RefreshCache::new(fetcher);

        let failed = cache.get(now).await.unwrap();
        // `failed` is still the current entry, so this refetches a replacement.
        let replacement = cache.refresh(now, &failed).await.unwrap();
        assert!(!Arc::ptr_eq(&failed, &replacement));

        // A second caller that also failed with the original now finds a newer,
        // fresh entry — it reuses it rather than fetching again.
        let reused = cache.refresh(now, &failed).await.unwrap();

        assert_eq!(calls.load(Ordering::SeqCst), 2);
        assert!(Arc::ptr_eq(&replacement, &reused));
        assert_eq!(*reused, 2);
    }

    #[tokio::test]
    async fn refresh_rechecks_staleness_of_replacement() {
        let now = SystemTime::UNIX_EPOCH;
        let (fetcher, calls) = TestFetcher::new(vec![
            Ok((1, now + HOUR)),
            // Replacement is already stale: its deadline is `now`.
            Ok((2, now)),
            Ok((3, now + HOUR)),
        ]);
        let cache = RefreshCache::new(fetcher);

        let failed = cache.get(now).await.unwrap();

        // Installs a replacement that is already stale, modelling a refresh we
        // blocked behind for so long it aged out before we got the lock.
        let stale_replacement = cache.refresh(now, &failed).await.unwrap();
        assert!(!Arc::ptr_eq(&failed, &stale_replacement));

        // The entry differs from the one we failed with — but it is stale, so
        // refresh must refetch rather than hand back the stale one.
        let refetched = cache.refresh(now, &failed).await.unwrap();

        assert_eq!(calls.load(Ordering::SeqCst), 3);
        assert!(!Arc::ptr_eq(&stale_replacement, &refetched));
        assert_eq!(*refetched, 3);
    }

    #[tokio::test]
    async fn fetch_error_leaves_cache_empty() {
        let now = SystemTime::UNIX_EPOCH;
        let (fetcher, calls) = TestFetcher::new(vec![Err(FetchError), Ok((5, now + HOUR))]);
        let cache = RefreshCache::new(fetcher);

        let result = cache.get(now).await;
        assert_eq!(result.unwrap_err(), FetchError);
        assert_eq!(calls.load(Ordering::SeqCst), 1);

        // The failed fetch left nothing cached, so the next get fetches afresh.
        let value = cache.get(now).await.unwrap();
        assert_eq!(*value, 5);
        assert_eq!(calls.load(Ordering::SeqCst), 2);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn concurrent_refresh_after_failure_fetches_once() {
        let now = SystemTime::UNIX_EPOCH;
        let fresh = now + HOUR;
        let (mut fetcher, calls) = TestFetcher::new(vec![Ok((1, fresh)), Ok((2, fresh))]);
        // The seed fetch (call 0) runs free; the elected refetch (call 1) blocks
        // on the gate so the losing racers pile up on the cache lock behind it.
        fetcher.gate_from = 1;
        let gate = Arc::clone(&fetcher.gate);
        let cache = Arc::new(RefreshCache::new(fetcher));

        let failed = cache.get(now).await.unwrap();
        assert_eq!(calls.load(Ordering::SeqCst), 1);

        const RACERS: usize = 32;
        let barrier = Arc::new(Barrier::new(RACERS));

        let mut handles = Vec::with_capacity(RACERS);
        for _ in 0..RACERS {
            let cache = Arc::clone(&cache);
            let failed = Arc::clone(&failed);
            let barrier = Arc::clone(&barrier);

            handles.push(tokio::spawn(async move {
                barrier.wait().await;
                cache.refresh(now, &failed).await.unwrap()
            }));
        }

        // Wait until the elected refetch has begun (call 1 on top of the seed),
        // then release it. `notify_one` stores a permit, so there is no
        // lost-wakeup race if the winner has not parked yet.
        while calls.load(Ordering::SeqCst) < 2 {
            tokio::task::yield_now().await;
        }
        gate.notify_one();

        let mut results = Vec::with_capacity(RACERS);
        for handle in handles {
            results.push(handle.await.unwrap());
        }

        // Exactly one refetch happened — the seed plus a single elected refresh.
        assert_eq!(calls.load(Ordering::SeqCst), 2);

        let winner = &results[0];
        for result in &results {
            assert!(Arc::ptr_eq(result, winner));
        }
        assert!(!Arc::ptr_eq(winner, &failed));
        assert_eq!(**winner, 2);
    }
}
