use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use lru::LruCache;
use parking_lot::Mutex;
use std::num::NonZeroUsize;
use tokio::sync::Notify;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Freshness {
    Fresh,
    Stale,
    Expired,
}

struct CacheEntry<T> {
    data: T,
    inserted: Instant,
    last_accessed: Instant,
}

impl<T: Clone> CacheEntry<T> {
    fn freshness(&self, ttl: Duration) -> Freshness {
        let age = self.inserted.elapsed();
        if age < ttl {
            Freshness::Fresh
        } else if age < ttl * 3 {
            Freshness::Stale
        } else {
            Freshness::Expired
        }
    }

    fn needs_bg_refresh(&self, ttl: Duration) -> bool {
        self.inserted.elapsed() > ttl.mul_f32(0.75)
    }
}

pub struct TypedCache<T> {
    entries: Mutex<LruCache<String, CacheEntry<T>>>,
    pending: Mutex<HashMap<String, Arc<Notify>>>,
    ttl: Duration,
}

pub struct CacheResult<T> {
    pub data: T,
    pub freshness: Freshness,
}

impl<T: Clone + Send + 'static> TypedCache<T> {
    pub fn new(ttl: Duration, max_entries: usize) -> Self {
        Self {
            entries: Mutex::new(LruCache::new(
                NonZeroUsize::new(max_entries).expect("max_entries must be > 0"),
            )),
            pending: Mutex::new(HashMap::new()),
            ttl,
        }
    }

    pub fn get(&self, key: &str) -> Option<CacheResult<T>> {
        let mut entries = self.entries.lock();
        let entry = entries.get_mut(key)?;
        entry.last_accessed = Instant::now();
        Some(CacheResult {
            data: entry.data.clone(),
            freshness: entry.freshness(self.ttl),
        })
    }

    pub fn needs_refresh(&self, key: &str) -> bool {
        let entries = self.entries.lock();
        entries
            .peek(key)
            .map(|e| e.needs_bg_refresh(self.ttl))
            .unwrap_or(true)
    }

    pub fn insert(&self, key: String, data: T) {
        let mut entries = self.entries.lock();
        let now = Instant::now();
        entries.push(
            key,
            CacheEntry {
                data,
                inserted: now,
                last_accessed: now,
            },
        );
    }

    pub fn invalidate(&self, key: &str) {
        self.entries.lock().pop(key);
    }

    pub fn clear(&self) {
        self.entries.lock().clear();
        self.pending.lock().clear();
    }

    pub fn register_pending(&self, key: &str) -> Option<Arc<Notify>> {
        let mut pending = self.pending.lock();
        if let Some(existing) = pending.get(key) {
            Some(existing.clone())
        } else {
            pending.insert(key.to_string(), Arc::new(Notify::new()));
            None
        }
    }

    pub fn complete_pending(&self, key: &str) {
        let mut pending = self.pending.lock();
        if let Some(notify) = pending.remove(key) {
            notify.notify_waiters();
        }
    }
}

