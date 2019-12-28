#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;

/// Key-value store
///
/// Contains datastore and functions to set and get values.
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    /// Creates new key-value store.
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    /// Save value to key-value store.
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    /// Return value from key-value store.
    pub fn get(&mut self, key: String) -> Option<String> {
        self.map.get(&key).cloned()
    }

    /// Remove value from key-value store.
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}
