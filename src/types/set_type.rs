use std::collections::HashSet;

use crate::store::store::{
    StoreVal, global_store, read_lock, read_unlock, write_lock, write_unlock,
};

pub struct SetType;

impl SetType {
    /// Add members to the set stored at `key`.
    /// Returns the number of new elements added.
    pub fn sadd(key: &str, values: &[String]) -> i64 {
        write_lock();
        let store = global_store();
        let mut set: HashSet<String> = store
            .get(&key.to_string())
            .unwrap()
            .get_set()
            .unwrap()
            .to_owned();
        let mut count = 0;

        for value in values {
            if set.insert(value.clone()) {
                count += 1;
            }
        }

        store.set(key.to_string(), StoreVal::Set(set), None);
        write_unlock();
        count
    }

    /// Returns the number of elements in the set stored at `key`.
    pub fn scard(key: &str) -> i64 {
        read_lock();
        let store = global_store();
        let set: HashSet<String> = store
            .get(&key.to_string())
            .unwrap()
            .get_set()
            .unwrap()
            .to_owned();
        read_unlock();
        set.len() as i64
    }

    /// Returns the difference between the first set and all subsequent sets.
    pub fn sdiff(keys: &[String]) -> Vec<String> {
        read_lock();
        let store = global_store();
        if keys.is_empty() {
            read_unlock();
            return vec![];
        }

        let base_set: HashSet<String> = store.get(&keys[0]).unwrap().get_set().unwrap().to_owned();
        let mut result_set = base_set.clone();

        for key in &keys[1..] {
            let other_set: HashSet<String> = store.get(key).unwrap().get_set().unwrap().to_owned();
            for val in other_set {
                result_set.remove(&val);
            }
        }

        read_unlock();
        result_set.into_iter().collect()
    }

    /// Returns true if `value` is a member of the set stored at `key`.
    pub fn sismember(key: &str, value: &str) -> bool {
        read_lock();
        let store = global_store();
        let set: HashSet<String> = store
            .get(&key.to_string())
            .unwrap()
            .get_set()
            .unwrap()
            .to_owned();
        read_unlock();
        set.contains(value)
    }
}
