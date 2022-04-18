use std::collections::HashMap;

use super::Store;

pub struct MemoryKvStore {
    kvs: HashMap<String, String>,
}

impl MemoryKvStore {
    pub fn new() -> MemoryKvStore {
        MemoryKvStore {
            kvs: HashMap::new(),
        }
    }
}

impl Store for MemoryKvStore {
    fn set(&mut self, key: &str, value: &str) -> Option<String> {
        self.kvs.insert(key.into(), value.into())
    }

    fn get(&self, key: &str) -> Option<String> {
        self.kvs.get(key).cloned()
    }

    fn remove(&mut self, key: &str) -> Option<String> {
        self.kvs.remove(key)
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn get() {
        let mut store = MemoryKvStore::new();
        crate::store::test_get(&mut store);
    }

    #[test]
    fn set() {
        let mut store = MemoryKvStore::new();
        crate::store::test_set(&mut store);
    }

    #[test]
    fn remove() {
        let mut store = MemoryKvStore::new();
        crate::store::test_remove(&mut store);
    }
}
