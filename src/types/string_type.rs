use std::collections::HashMap;

use crate::store::store::{
    StoreVal, global_store, read_lock, read_unlock, write_lock, write_unlock,
};

#[derive(Default)]
pub struct StringType;

impl StringType {
    pub fn append(key: &str, value: &str) {
        write_lock();
        let store = global_store();
        let current = store.get(&key.to_string());
        let new_value = match current {
            Some(v) => v.get_str().unwrap().to_string() + value,
            None => value.to_string(),
        };
        store.set(key.to_string(), StoreVal::Str(new_value), None);
        write_unlock();
    }

    pub fn decr(key: &str) -> Result<(), String> {
        Self::decr_by(key, 1)
    }

    pub fn decr_by(key: &str, value: i64) -> Result<(), String> {
        write_lock();
        let store = global_store();

        let new_val = if let Some(StoreVal::Str(s)) = store.get(&key.to_string()) {
            s.parse::<i64>()
                .map_err(|_| "ERR value is not an integer or out of range".to_string())?
                - value
        } else {
            -value
        };

        store.set(key.to_string(), StoreVal::Str(new_val.to_string()), None);
        write_unlock();
        Ok(())
    }

    pub fn get(key: &str) -> Result<String, String> {
        read_lock();
        let store = global_store();
        let result = store
            .get(&key.to_string())
            .ok_or_else(|| "ERR key not found".to_string())?;
        read_unlock();
        Ok(result.get_str().unwrap().to_owned())
    }

    pub fn get_del(key: &str) -> Result<String, String> {
        write_lock();
        let store = global_store();
        let val = store
            .get(&key.to_string())
            .ok_or_else(|| "ERR key not found".to_string())?;
        store.delete(&key.to_string());
        write_unlock();
        Ok(val.get_str().unwrap().to_owned())
    }

    pub fn get_ex(key: &str, exp_seconds: u64) -> Result<String, String> {
        write_lock();
        let store = global_store();
        let val = store
            .get(&key.to_string())
            .ok_or_else(|| "ERR key not found".to_string())?;
        store.set(key.to_string(), val.clone(), Some(exp_seconds));
        write_unlock();
        Ok(val.get_str().unwrap().to_owned())
    }

    pub fn get_range(key: &str, start: isize, end: isize) -> Result<String, String> {
        read_lock();
        let store = global_store();
        let store_val = store
            .get(&key.to_string())
            .ok_or_else(|| "ERR key not found".to_string())?;
        let val = store_val.get_str().unwrap();
        let len = val.len() as isize;
        if len == 0 {
            read_unlock();
            return Ok(String::new());
        }
        let start_idx = ((start % len + len) % len) as usize;
        let end_idx = ((end % len + len) % len) as usize;
        if start_idx > end_idx {
            read_unlock();
            return Err("ERR start index greater than end index".to_string());
        }
        let slice = &val[start_idx..=end_idx];
        read_unlock();
        Ok(slice.to_string())
    }

    pub fn get_set(key: &str, value: &str) -> Result<String, String> {
        write_lock();
        let store = global_store();
        let old_val = store
            .get(&key.to_string())
            .ok_or_else(|| "ERR key not found".to_string())?;
        store.set(key.to_string(), StoreVal::Str(value.to_string()), None);
        write_unlock();
        Ok(old_val.get_str().unwrap().to_owned())
    }

    pub fn incr(key: &str) -> Result<(), String> {
        Self::incr_by(key, 1)
    }

    pub fn incr_by(key: &str, value: i64) -> Result<(), String> {
        write_lock();
        let store = global_store();
        let current = store.get(&key.to_string());
        let new_val = match current {
            Some(v) => {
                let int_val = v
                    .get_str()
                    .unwrap()
                    .parse::<i64>()
                    .map_err(|_| "ERR value is not an integer or out of range".to_string())?;
                int_val + value
            }
            None => value,
        };
        store.set(key.to_string(), StoreVal::Str(new_val.to_string()), None);
        write_unlock();
        Ok(())
    }

    pub fn set(key: &str, value: &str) {
        write_lock();
        let store = global_store();
        store.set(key.to_string(), StoreVal::Str(value.to_string()), None);
        write_unlock();
    }

    pub fn set_ex(key: &str, value: &str, seconds: u64) {
        write_lock();
        let store = global_store();
        store.set(
            key.to_string(),
            StoreVal::Str(value.to_string()),
            Some(seconds),
        );
        write_unlock();
    }

    pub fn str_len(key: &str) -> Result<usize, String> {
        read_lock();
        let store = global_store();
        let val = store
            .get(&key.to_string())
            .ok_or_else(|| "ERR key not found".to_string())?;
        read_unlock();
        Ok(val.get_str().unwrap().len())
    }

    pub fn mget(keys: &[String]) -> Vec<String> {
        read_lock();
        let store = global_store();
        let values = keys
            .iter()
            .map(|k| store.get(k).unwrap().get_str().unwrap().to_owned())
            .collect();
        read_unlock();
        values
    }

    pub fn mset(kv_pairs: &HashMap<String, String>) {
        write_lock();
        let store = global_store();
        for (k, v) in kv_pairs {
            store.set(k.clone(), StoreVal::Str(v.clone()), None);
        }
        write_unlock();
    }

    pub fn lcs(key1: &str, key2: &str, command: Option<&str>) -> Result<String, String> {
        read_lock();
        let store = global_store();
        let val1 = store
            .get(&key1.to_string())
            .ok_or_else(|| "ERR key not found".to_string())?;
        let val2 = store
            .get(&key2.to_string())
            .ok_or_else(|| "ERR key not found".to_string())?;
        read_unlock();

        let (lcs_str, lcs_len) = find_lcs(&val1.get_str().unwrap(), &val2.get_str().unwrap());
        if let Some(cmd) = command {
            if cmd.eq_ignore_ascii_case("LEN") {
                return Ok(lcs_len.to_string());
            }
        }
        Ok(lcs_str)
    }
}

/// Find Longest Common Subsequence
fn find_lcs(a: &str, b: &str) -> (String, usize) {
    let m = a.len();
    let n = b.len();
    let mut prev = vec![0; n + 1];
    let mut curr = vec![0; n + 1];

    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();

    for i in 0..m {
        for j in 0..n {
            if a_bytes[i] == b_bytes[j] {
                curr[j + 1] = prev[j] + 1;
            } else {
                curr[j + 1] = prev[j + 1].max(curr[j]);
            }
        }
        std::mem::swap(&mut prev, &mut curr);
    }

    let mut lcs_len = prev[n];
    let mut lcs = vec![0u8; lcs_len];
    let mut i = m;
    let mut j = n;
    while i > 0 && j > 0 {
        if a_bytes[i - 1] == b_bytes[j - 1] {
            lcs_len -= 1;
            lcs[lcs_len] = a_bytes[i - 1];
            i -= 1;
            j -= 1;
        } else if prev[j] > prev[j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }
    (String::from_utf8(lcs).unwrap(), prev[n])
}
