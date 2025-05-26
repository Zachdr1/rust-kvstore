use super::kvstore::{Backend, KeyValueStore};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{read_to_string, File, OpenOptions};
use std::io::{Read, SeekFrom, Write};
use std::path::Path;

pub struct HashMapBackend<K, V>
where
    K: for<'a> Deserialize<'a> + Serialize + std::cmp::Eq + std::hash::Hash,
    V: for<'a> Deserialize<'a> + Serialize,
{
    data: HashMap<K, V>,
    file: std::fs::File,
}

impl<K, V> Backend<K, V> for HashMapBackend<K, V>
where
    K: for<'a> Deserialize<'a> + Serialize + std::cmp::Eq + std::hash::Hash,
    V: for<'a> Deserialize<'a> + Serialize,
{
    fn new(filepath: &str) -> Self {
        let file_content = std::fs::read_to_string(filepath).unwrap_or_default();

        let data: HashMap<K, V> = if file_content.is_empty() {
            HashMap::new()
        } else {
            serde_json::from_str(&file_content).unwrap_or_else(|_| HashMap::new())
        };

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true) // Create if doesn't exist
            .truncate(true)
            .open(filepath)
            .expect("Cant open file");

        Self { data, file }
    }

    fn insert(&mut self, key: K, val: V) -> Result<Option<V>, std::io::Error> {
        let res = self.data.insert(key, val);
        let json = serde_json::to_string(&self.data)?;
        writeln!(self.file, "{}", json)?;
        Ok(res)
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.data.remove(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashmap_backend() {
        let mut store: KeyValueStore<HashMapBackend<String, i32>, String, i32> =
            KeyValueStore::new("test.json");

        // Insert some data
        store.insert("key1".to_string(), 42);
        store.insert("key2".to_string(), 100);

        // Create new store and load
        let mut store2: KeyValueStore<HashMapBackend<String, i32>, String, i32> =
            KeyValueStore::new("test.json");

        // Verify data was loaded
        assert_eq!(store2.get(&"key1".to_string()), Some(&42));
        assert_eq!(store2.get(&"key2".to_string()), Some(&100));
    }
}
