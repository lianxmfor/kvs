use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::lsm::mem_table::MemTable;
use crate::lsm::wal::WAL;
use crate::Store;

#[derive(Debug)]
pub struct DatabaseEntry {
    key: Vec<u8>,
    value: Vec<u8>,
    timestamp: u128,
}

impl DatabaseEntry {
    pub fn key(&self) -> &[u8] {
        &self.key
    }

    pub fn value(&self) -> &[u8] {
        &self.value
    }

    pub fn timestamp(&self) -> u128 {
        self.timestamp
    }
}

pub struct Database {
    dir: PathBuf,
    mem_table: MemTable,
    wal: WAL,
}

impl Database {
    pub fn new(dir: PathBuf) -> Database {
        let (wal, mem_table) = WAL::load_from_dir(&dir).unwrap();

        Database {
            dir,
            mem_table,
            wal,
        }
    }

    pub fn dir(&self) -> &PathBuf {
        &self.dir
    }
}

impl Store for Database {
    fn get(&self, key: &[u8]) -> crate::Result<Option<&[u8]>> {
        match self.mem_table.get(key) {
            Some(entry) => Ok(entry.value.as_deref()),
            None => Ok(None),
        }
    }

    fn set(&mut self, key: &[u8], value: &[u8]) -> crate::Result<()> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros();

        self.wal.set(key, value, timestamp)?;
        self.mem_table.set(key, value, timestamp);

        Ok(())
    }

    fn remove(&mut self, key: &[u8]) -> crate::Result<()> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros();

        self.wal.remove(key, timestamp)?;
        self.mem_table.delete(key, timestamp);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn get() {
        let dir = tempdir().unwrap();
        let mut store = Database::new(dir.path().into());
        crate::store::test_get(&mut store);
    }

    #[test]
    fn set() {
        let dir = tempdir().unwrap();
        let mut store = Database::new(dir.path().into());
        crate::store::test_set(&mut store);
    }

    #[test]
    fn remove() {
        let dir = tempdir().unwrap();
        let mut store = Database::new(dir.path().into());
        crate::store::test_remove(&mut store);
    }
}
