pub mod lsm;
pub mod memory;

use crate::Result;

pub trait Store {
    /// Sets the value of a string key to a string.
    ///
    /// If the key already exists, the previous value will be overwritten.
    ///
    /// # Errors
    ///
    /// It propagates I/O or serialization errors during writing the log.
    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<()>;

    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    ///
    /// # Errors
    ///
    /// It returns `KvsError::UnexpectedCommandType` if the given command type unexpected.
    fn get(&self, key: &[u8]) -> Result<Option<&[u8]>>;

    /// Removes a given key.
    ///
    /// # Errors
    ///
    /// It returns `KvsError::KeyNotFound` if the given key is not found.
    ///
    /// It propagates I/O or serialization errors during writing the log.
    fn remove(&mut self, key: &[u8]) -> Result<()>;
}

#[cfg(test)]
fn test_set(store: &mut impl Store) {
    let v = store.set(b"k1", b"v").unwrap();
    assert_eq!(v, ());

    let v = store.set(b"k1", b"v1").unwrap();
    assert_eq!(v, ());
}

#[cfg(test)]
fn test_get(store: &mut impl Store) {
    let v = store.get(b"k1").unwrap();
    assert!(v.is_none());

    assert_eq!(store.set(b"k1", b"v1").unwrap(), ());

    let v = store.get(b"k1").unwrap().unwrap();
    assert_eq!(v, b"v1");
}

#[cfg(test)]
fn test_remove(store: &mut impl Store) {
    assert_eq!(store.remove(b"k1").unwrap(), ());
    assert_eq!(store.set(b"k1", b"v1").unwrap(), ());

    let v = store.get(b"k1").unwrap().unwrap();
    assert_eq!(v, b"v1");

    let v = store.remove(b"k1").unwrap();
    assert_eq!(v, ());

    let v = store.get(b"k1").unwrap();
    assert_eq!(v, None);
}
