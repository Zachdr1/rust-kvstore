use super::kvstore::{Backend, KeyValueStore};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::path::Path;

pub struct HashMapBackend<K, V>
where
    K: for<'a> Deserialize<'a> + Serialize + std::cmp::Eq + std::hash::Hash,
    V: for<'a> Deserialize<'a> + Serialize,
{
    data: HashMap<K, V>,
    filepath: String,
}

impl<K, V> Backend<K, V> for HashMapBackend<K, V>
where
    K: for<'a> Deserialize<'a> + Serialize + std::cmp::Eq + std::hash::Hash,
    V: for<'a> Deserialize<'a> + Serialize + Copy,
{
    fn new(filepath: &str) -> Self {
        let data: HashMap<K, V>;
        if !Path::new(filepath).exists() {
            let _ = File::create(filepath).expect("Unable to create file");
            data = HashMap::<K, V>::new();
        } else {
            let file_content = read_to_string(filepath).expect("Unable to read file contents");
            if !file_content.is_empty() {
                println!("Data found in {}, loading..", filepath);
                data = serde_json::from_str(&file_content).expect("Unable to parse data");
            } else {
                println!("{} empty", filepath);
                data = HashMap::<K, V>::new();
            }
        }

        Self {
            data,
            filepath: filepath.to_string(),
        }
    }

    fn insert(&mut self, key: K, val: V) -> Result<Option<V>, std::io::Error> {
        let res = self.data.insert(key, val);
        let json = serde_json::to_string(&self.data)?;
        std::fs::write(&self.filepath, json)?;
        Ok(res)
    }

    fn get(&mut self, key: &K) -> Option<V> {
        Some(self.data.get(key)?.clone())
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.data.remove(key)
    }

    fn flush(&self) -> Result<(), std::io::Error> {
        let json = serde_json::to_string(&self.data)?;
        std::fs::write(&self.filepath, json)?;
        Ok(())
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
