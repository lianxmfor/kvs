/// MemTable entry.
pub struct MemTableEntry {
    pub key: Vec<u8>,
    pub value: Option<Vec<u8>>,
    pub timestamp: u128,
    pub deleted: bool,
}

/// MemTable holds a sorted list of the latest written records.
///
/// Writes are duplicated to the WAL for recovery of the MemTable in the event of a restart.
///
/// MemTables has max capacity and when that is reached, we flush the MemTable to disk as a Table(SSTABLE)
///
/// Entries are sorted in a Vector instead of a HashMap to support Scans.
#[derive(Default)]
pub struct MemTable {
    entries: Vec<MemTableEntry>,
    size: usize,
}

impl MemTable {
    pub fn new() -> MemTable {
        MemTable::default()
    }

    /// Performs Binary Search to find a record in the MemTable.
    ///
    /// If the record is found `[Result::Ok]` is returned, with the index of record. If the record is not
    ///
    /// found then `[Result::Err]` is returned, with the index to insert the record at.
    pub fn get_index(&self, key: &[u8]) -> std::result::Result<usize, usize> {
        self.entries
            .binary_search_by_key(&key, |e| e.key.as_slice())
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

impl MemTable {
    pub fn get(&self, key: &[u8]) -> Option<&MemTableEntry> {
        match self.get_index(key) {
            Ok(idx) => Some(&self.entries[idx]),
            Err(_) => None,
        }
    }

    pub fn set(&mut self, key: &[u8], value: &[u8], timestamp: u128) {
        let entry = MemTableEntry {
            key: key.to_owned(),
            value: Some(value.to_owned()),
            timestamp,
            deleted: false,
        };

        match self.get_index(key) {
            Ok(idx) => {
                match self.entries[idx].value.as_ref() {
                    Some(v) => {
                        self.size += value.len();
                        self.size -= v.len();
                    }
                    None => self.size += value.len(),
                }
                self.entries[idx] = entry;
            }
            Err(idx) => {
                self.entries.insert(idx, entry);

                // Increase the size of the MemTale by the key size, Value size, Timestamp
                // size (16 byte), Tombstone size(1 byte).
                self.size += key.len() + value.len() + 16 + 1;
            }
        }
    }

    pub fn delete(&mut self, key: &[u8], timestamp: u128) {
        let entry = MemTableEntry {
            key: key.to_owned(),
            value: None,
            timestamp,
            deleted: true,
        };

        match self.get_index(key) {
            Ok(idx) => {
                if let Some(v) = self.entries[idx].value.as_ref() {
                    self.size -= v.len();
                }
                self.entries[idx] = entry;
            }
            Err(idx) => {
                // Increase the size of the MemTale by the key size, Value size, Timestamp
                // size (16 byte), Tombstone size(1 byte).
                self.size += key.len() + 16 + 1;
                self.entries.insert(idx, entry);
            }
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    #[test]
    fn mem_table_put_start() {
        let mut table = MemTable::new();
        table.set(b"Lime", b"Lime Smoothie", 0); // 17 + 16 + 1
        table.set(b"Orange", b"Orange Smoothie", 10); // 21 + 16 + 1

        table.set(b"Apple", b"Apple Smoothie", 20); // 19 + 16 + 1

        assert_eq!(table.entries[0].key, b"Apple");
        assert_eq!(table.entries[0].value.as_ref().unwrap(), b"Apple Smoothie");
        assert_eq!(table.entries[0].timestamp, 20);
        assert_eq!(table.entries[0].deleted, false);
        assert_eq!(table.entries[1].key, b"Lime");
        assert_eq!(table.entries[1].value.as_ref().unwrap(), b"Lime Smoothie");
        assert_eq!(table.entries[1].timestamp, 0);
        assert_eq!(table.entries[1].deleted, false);
        assert_eq!(table.entries[2].key, b"Orange");
        assert_eq!(table.entries[2].value.as_ref().unwrap(), b"Orange Smoothie");
        assert_eq!(table.entries[2].timestamp, 10);
        assert_eq!(table.entries[2].deleted, false);

        assert_eq!(table.size, 108);
    }

    #[test]
    fn mem_table_put_middle() {
        let mut table = MemTable::new();
        table.set(b"Apple", b"Apple Smoothie", 0);
        table.set(b"Orange", b"Orange Smoothie", 10);

        table.set(b"Lime", b"Lime Smoothie", 20);

        assert_eq!(table.entries[0].key, b"Apple");
        assert_eq!(table.entries[0].value.as_ref().unwrap(), b"Apple Smoothie");
        assert_eq!(table.entries[0].timestamp, 0);
        assert_eq!(table.entries[0].deleted, false);
        assert_eq!(table.entries[1].key, b"Lime");
        assert_eq!(table.entries[1].value.as_ref().unwrap(), b"Lime Smoothie");
        assert_eq!(table.entries[1].timestamp, 20);
        assert_eq!(table.entries[1].deleted, false);
        assert_eq!(table.entries[2].key, b"Orange");
        assert_eq!(table.entries[2].value.as_ref().unwrap(), b"Orange Smoothie");
        assert_eq!(table.entries[2].timestamp, 10);
        assert_eq!(table.entries[2].deleted, false);

        assert_eq!(table.size, 108);
    }

    #[test]
    fn mem_table_put_end() {
        let mut table = MemTable::new();
        table.set(b"Apple", b"Apple Smoothie", 0);
        table.set(b"Lime", b"Lime Smoothie", 10);

        table.set(b"Orange", b"Orange Smoothie", 20);

        assert_eq!(table.entries[0].key, b"Apple");
        assert_eq!(table.entries[0].value.as_ref().unwrap(), b"Apple Smoothie");
        assert_eq!(table.entries[0].timestamp, 0);
        assert_eq!(table.entries[0].deleted, false);
        assert_eq!(table.entries[1].key, b"Lime");
        assert_eq!(table.entries[1].value.as_ref().unwrap(), b"Lime Smoothie");
        assert_eq!(table.entries[1].timestamp, 10);
        assert_eq!(table.entries[1].deleted, false);
        assert_eq!(table.entries[2].key, b"Orange");
        assert_eq!(table.entries[2].value.as_ref().unwrap(), b"Orange Smoothie");
        assert_eq!(table.entries[2].timestamp, 20);
        assert_eq!(table.entries[2].deleted, false);

        assert_eq!(table.size, 108);
    }

    #[test]
    fn mem_table_put_overwrite() {
        let mut table = MemTable::new();
        table.set(b"Apple", b"Apple Smoothie", 0);
        table.set(b"Lime", b"Lime Smoothie", 10);
        table.set(b"Orange", b"Orange Smoothie", 20);

        table.set(b"Lime", b"A sour fruit", 30);

        assert_eq!(table.entries[0].key, b"Apple");
        assert_eq!(table.entries[0].value.as_ref().unwrap(), b"Apple Smoothie");
        assert_eq!(table.entries[0].timestamp, 0);
        assert_eq!(table.entries[0].deleted, false);
        assert_eq!(table.entries[1].key, b"Lime");
        assert_eq!(table.entries[1].value.as_ref().unwrap(), b"A sour fruit");
        assert_eq!(table.entries[1].timestamp, 30);
        assert_eq!(table.entries[1].deleted, false);
        assert_eq!(table.entries[2].key, b"Orange");
        assert_eq!(table.entries[2].value.as_ref().unwrap(), b"Orange Smoothie");
        assert_eq!(table.entries[2].timestamp, 20);
        assert_eq!(table.entries[2].deleted, false);

        assert_eq!(table.size, 107);
    }

    #[test]
    fn mem_table_get_exists() {
        let mut table = MemTable::new();
        table.set(b"Apple", b"Apple Smoothie", 0);
        table.set(b"Lime", b"Lime Smoothie", 10);
        table.set(b"Orange", b"Orange Smoothie", 20);

        let entry = table.get(b"Orange").unwrap();

        assert_eq!(entry.key, b"Orange");
        assert_eq!(entry.value.as_ref().unwrap(), b"Orange Smoothie");
        assert_eq!(entry.timestamp, 20);
    }

    #[test]
    fn mem_table_get_not_exists() {
        let mut table = MemTable::new();
        table.set(b"Apple", b"Apple Smoothie", 0);
        table.set(b"Lime", b"Lime Smoothie", 0);
        table.set(b"Orange", b"Orange Smoothie", 0);

        let res = table.get(b"Potato");
        assert_eq!(res.is_some(), false);
    }

    #[test]
    fn mem_table_delete_exists() {
        let mut table = MemTable::new();
        table.set(b"Apple", b"Apple Smoothie", 0);

        table.delete(b"Apple", 10);

        let res = table.get(b"Apple").unwrap();
        assert_eq!(res.key, b"Apple");
        assert_eq!(res.value, None);
        assert_eq!(res.timestamp, 10);
        assert_eq!(res.deleted, true);

        assert_eq!(table.entries[0].key, b"Apple");
        assert_eq!(table.entries[0].value, None);
        assert_eq!(table.entries[0].timestamp, 10);
        assert_eq!(table.entries[0].deleted, true);

        assert_eq!(table.size, 22);
    }

    #[test]
    fn mem_table_delete_empty() {
        let mut table = MemTable::new();

        table.delete(b"Apple", 10);

        let res = table.get(b"Apple").unwrap();
        assert_eq!(res.key, b"Apple");
        assert_eq!(res.value, None);
        assert_eq!(res.timestamp, 10);
        assert_eq!(res.deleted, true);

        assert_eq!(table.entries[0].key, b"Apple");
        assert_eq!(table.entries[0].value, None);
        assert_eq!(table.entries[0].timestamp, 10);
        assert_eq!(table.entries[0].deleted, true);

        assert_eq!(table.size, 22);
    }
}
