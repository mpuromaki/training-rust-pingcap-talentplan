#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;

pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    // Create new store
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    // Set value to a key.
    pub fn set(&mut self, key: String, value: String) {
        panic!()
    }

    // Get value of a key.
    pub fn get(&mut self, key: String) -> Option<String> {
        panic!()
    }

    // Remove key and its value.
    pub fn remove(&mut self, key: String) {
        panic!()
    }
}
