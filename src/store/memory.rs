use std::collections::HashMap;

use super::Store;
use crate::Result;

pub struct MemoryKvStore {
    kvs: HashMap<Vec<u8>, Vec<u8>>,
}

impl MemoryKvStore {
    pub fn new() -> MemoryKvStore {
        MemoryKvStore {
            kvs: HashMap::new(),
        }
    }
}

impl Store for MemoryKvStore {
    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<()> {
        self.kvs.insert(key.into(), value.into());
        Ok(())
    }

    fn get(&self, key: &[u8]) -> Result<Option<&[u8]>> {
        let value = self.kvs.get(key).map(Vec::as_ref);

        Ok(value)
    }

    fn remove(&mut self, key: &[u8]) -> Result<()> {
        self.kvs.remove(key);
        Ok(())
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
