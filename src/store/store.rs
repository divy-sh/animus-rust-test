// use std::hash::Hash;
// use std::num::NonZero;
// use std::sync::{Once, RwLock};
// use std::thread;
// use std::time::{Duration, SystemTime, UNIX_EPOCH};

// use lru::LruCache;

// /// Value stored in the cache with optional TTL
// #[derive(Clone)]
// struct Value<V> {
//     val: V,
//     ttl: Option<u64>, // Unix timestamp in seconds
// }

// /// The Store itself
// pub struct Store<K, V>
// where
//     K: Eq + Hash + Clone,
// {
//     cache: RwLock<LruCache<K, Value<V>>>,
//     max_size: usize,
// }

// /// A guard that allows safe access to the store
// pub struct StoreLock;

// impl StoreLock {
//     /// Acquire the global store lock
//     pub fn lock() -> StoreLock {
//         GLOBAL_LOCK.write().unwrap();
//         StoreLock
//     }
// }

// // Automatically unlock when guard drops
// impl Drop for StoreLock {
//     fn drop(&mut self) {
//         drop(GLOBAL_LOCK.write().unwrap());
//     }
// }

// static mut GLOBAL_STORE_PTR: *const Store<String, String> = 0 as *const Store<String, String>;
// static INIT: Once = Once::new();
// static GLOBAL_LOCK: RwLock<()> = RwLock::new(());

// /// Access the global store
// pub fn global_store() -> &'static Store<String, String> {
//     unsafe {
//         INIT.call_once(|| {
//             let store = Store {
//                 cache: RwLock::new(LruCache::new(NonZero::new(100_000).unwrap())),
//                 max_size: 100_000,
//             };

//             // Start cleaner thread
//             let store_ptr: *const Store<String, String> = &store;
//             thread::spawn(move || {
//                 loop {
//                     thread::sleep(Duration::from_secs(1));
//                     let store_ref: &Store<String, String> = unsafe { &*store_ptr };
//                     store_ref.clean_expired();
//                 }
//             });

//             GLOBAL_STORE_PTR = Box::into_raw(Box::new(store));
//         });
//         &*GLOBAL_STORE_PTR
//     }
// }

// impl<K, V> Store<K, V>
// where
//     K: Eq + Hash + Clone,
//     V: Clone,
// {
//     fn clean_expired(&self) {
//         let mut cache = self.cache.write().unwrap();
//         let now = current_unix_time();
//         let keys_to_remove: Vec<K> = cache
//             .iter()
//             .filter_map(|(k, v)| match v.ttl {
//                 Some(ttl) if ttl <= now => Some(k.clone()),
//                 _ => None,
//             })
//             .collect();

//         for key in keys_to_remove {
//             cache.pop(&key);
//         }
//     }

//     pub fn get(&self, _lock: &StoreLock, key: &K) -> Option<V> {
//         let mut cache = self.cache.write().unwrap();
//         if let Some(value) = cache.get(key) {
//             if let Some(ttl) = value.ttl {
//                 if ttl <= current_unix_time() {
//                     cache.pop(key);
//                     return None;
//                 }
//             }
//             return Some(value.val.clone());
//         }
//         None
//     }

//     pub fn set(&self, _lock: &StoreLock, key: K, val: V, ttl_seconds: Option<u64>) {
//         let ttl = ttl_seconds.map(|t| t + current_unix_time());
//         let mut cache = self.cache.write().unwrap();
//         cache.put(key, Value { val, ttl });
//     }

//     pub fn delete(&self, _lock: &StoreLock, key: &K) {
//         let mut cache = self.cache.write().unwrap();
//         cache.pop(key);
//     }

//     pub fn keys(&self, _lock: &StoreLock) -> Vec<K> {
//         let cache = self.cache.read().unwrap();
//         cache.iter().map(|(k, _)| k.clone()).collect()
//     }
// }

// fn current_unix_time() -> u64 {
//     SystemTime::now()
//         .duration_since(UNIX_EPOCH)
//         .unwrap()
//         .as_secs()
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_store_singleton() {
//         let lock = StoreLock::lock();
//         let store = global_store();

//         store.set(&lock, "key1".to_string(), "value1".to_string(), Some(1));
//         assert_eq!(
//             store.get(&lock, &"key1".to_string()),
//             Some("value1".to_string())
//         );
//         std::thread::sleep(Duration::from_secs(2));
//         assert_eq!(store.get(&lock, &"key1".to_string()), None);

//         let keys = store.keys(&lock);
//         assert!(keys.is_empty());
//     }
// }
