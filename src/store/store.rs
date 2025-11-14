use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::num::NonZero;
use std::sync::{Arc, OnceLock, RwLock};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use lru::LruCache;

#[derive(Clone)]
pub enum StoreVal {
    Str(String),
    Hash(HashMap<String, String>),
    Set(HashSet<String>),
    List(Vec<String>),
}

impl StoreVal {
    pub fn get_str(&self) -> Option<&String> {
        if let StoreVal::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn get_hash(&self) -> Option<&HashMap<String, String>> {
        if let StoreVal::Hash(h) = self {
            Some(h)
        } else {
            None
        }
    }

    pub fn get_set(&self) -> Option<&HashSet<String>> {
        if let StoreVal::Set(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn get_list(&self) -> Option<&Vec<String>> {
        if let StoreVal::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
}

/// Value stored in the cache with optional TTL
#[derive(Clone)]
struct Value<V> {
    val: V,
    ttl: Option<u64>, // Unix timestamp in seconds
}

/// The Store itself
pub struct Store<K, V>
where
    K: Eq + Hash + Clone,
{
    cache: RwLock<LruCache<K, Value<V>>>,
}

static GLOBAL_STORE: OnceLock<Arc<Store<String, StoreVal>>> = OnceLock::new();
static GLOBAL_LOCK: RwLock<()> = RwLock::new(());

// Global guards to implement explicit read/write lock API
static mut READ_GUARD: Option<std::sync::RwLockReadGuard<'static, ()>> = None;
static mut WRITE_GUARD: Option<std::sync::RwLockWriteGuard<'static, ()>> = None;

/// Acquire a read lock
pub fn read_lock() {
    unsafe {
        READ_GUARD = Some(GLOBAL_LOCK.read().unwrap());
    }
}

/// Release a read lock
pub fn read_unlock() {
    unsafe {
        READ_GUARD = None; // dropping the guard unlocks
    }
}

/// Acquire a write lock
pub fn write_lock() {
    unsafe {
        WRITE_GUARD = Some(GLOBAL_LOCK.write().unwrap());
    }
}

/// Release a write lock
pub fn write_unlock() {
    unsafe {
        WRITE_GUARD = None; // dropping the guard unlocks
    }
}

/// Access the global store
pub fn global_store() -> &'static Arc<Store<String, StoreVal>> {
    GLOBAL_STORE.get_or_init(|| {
        let store = Arc::new(Store {
            cache: RwLock::new(LruCache::new(NonZero::new(100_000).unwrap())),
        });

        // Background cleaner thread
        let store_clone = store.clone();
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(1));
                store_clone.clean_expired();
            }
        });

        store
    })
}

impl<K, V> Store<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn clean_expired(&self) {
        let mut cache = self.cache.write().unwrap();
        let now = current_unix_time();
        let keys_to_remove: Vec<K> = cache
            .iter()
            .filter_map(|(k, v)| match v.ttl {
                Some(ttl) if ttl <= now => Some(k.clone()),
                _ => None,
            })
            .collect();

        for key in keys_to_remove {
            cache.pop(&key);
        }
    }

    pub fn get(&self, key: &K) -> Option<V> {
        let mut cache = self.cache.write().unwrap();
        if let Some(value) = cache.get(key) {
            if let Some(ttl) = value.ttl {
                if ttl <= current_unix_time() {
                    cache.pop(key);
                    return None;
                }
            }
            return Some(value.val.clone());
        }
        None
    }

    pub fn set(&self, key: K, val: V, ttl_seconds: Option<u64>) {
        let ttl = ttl_seconds.map(|t| t + current_unix_time());
        let mut cache = self.cache.write().unwrap();
        cache.put(key, Value { val, ttl });
    }

    pub fn delete(&self, key: &K) {
        let mut cache = self.cache.write().unwrap();
        cache.pop(key);
    }

    pub fn keys(&self) -> Vec<K> {
        let cache = self.cache.read().unwrap();
        cache.iter().map(|(k, _)| k.clone()).collect()
    }
}

fn current_unix_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
