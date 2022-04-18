pub mod memory;

pub trait Store {
    /// Sets the values of a string key to a string.
    ///
    /// If the key already exists, the previous value will be overwritten.
    fn set(&mut self, key: &str, value: &str) -> Option<String>;

    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    fn get(&self, key: &str) -> Option<String>;

    /// Remove a given key
    fn remove(&mut self, key: &str) -> Option<String>;
}

#[cfg(test)]
fn test_set(store: &mut impl Store) {
    let v = store.set("k1", "v");
    assert_eq!(v, None);

    let v = store.set("k1", "v1");
    assert_eq!(v, Some("v".to_string()))
}

#[cfg(test)]
fn test_get(store: &mut impl Store) {
    let v = store.get("k1");
    assert_eq!(v, None);

    store.set("k1", "v1");

    let v = store.get("k1");
    assert_eq!(v, Some("v1".to_string()));
}

#[cfg(test)]
fn test_remove(store: &mut impl Store) {
    store.set("k1", "v1");
    let v = store.get("k1");
    assert_eq!(v, Some("v1".to_string()));

    let v = store.remove("k1");
    assert_eq!(v, Some("v1".to_string()));

    let v = store.get("k1");
    assert_eq!(v, None);
}
