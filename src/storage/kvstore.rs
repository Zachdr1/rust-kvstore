use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::path::Path;

pub trait Backend<K, V> {
    fn new(filepath: &str) -> Self;
    fn insert(&mut self, key: K, val: V) -> Option<V>;
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
    fn save(&self) -> Result<(), std::io::Error>;
    fn load(&mut self) -> Result<(), std::io::Error>;
}

pub struct KeyValueStore<B, K, V>
where
    B: Backend<K, V>,
{
    backend: B,
    _phantom: std::marker::PhantomData<(K, V)>,
}

impl<B, K, V> KeyValueStore<B, K, V>
where
    B: Backend<K, V>,
{
    pub fn new(filepath: &str) -> Self {
        Self {
            backend: B::new(filepath),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn insert(&mut self, key: K, val: V) -> Option<V> {
        self.backend.insert(key, val)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.backend.get(key)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.backend.remove(key)
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        self.backend.save()
    }

    pub fn load(&mut self) -> Result<(), std::io::Error> {
        self.backend.load()
    }
}

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
    V: for<'a> Deserialize<'a> + Serialize,
{
    fn new(filepath: &str) -> Self {
        if !Path::new(filepath).exists() {
            let _ = File::create(filepath).unwrap();
        }

        let data = HashMap::<K, V>::new();

        Self {
            data,
            filepath: filepath.to_string(),
        }
    }

    fn insert(&mut self, key: K, val: V) -> Option<V> {
        self.data.insert(key, val)
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.data.remove(key)
    }

    fn save(&self) -> Result<(), std::io::Error> {
        let json = serde_json::to_string(&self.data)?;
        std::fs::write(&self.filepath, json)?;
        Ok(())
    }

    fn load(&mut self) -> Result<(), std::io::Error> {
        let file_content = read_to_string(&self.filepath)?;
        if !file_content.is_empty() {
            println!("Data found in {}, loading..", &self.filepath);
            self.data = serde_json::from_str(&file_content)?;
            Ok(())
        } else {
            println!("{} empty", &self.filepath);
            Ok(())
        }
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

        // Save to file
        store.save().unwrap();

        // Create new store and load
        let mut store2: KeyValueStore<HashMapBackend<String, i32>, String, i32> =
            KeyValueStore::new("test.json");
        store2.load().unwrap();

        // Verify data was loaded
        assert_eq!(store2.get(&"key1".to_string()), Some(&42));
        assert_eq!(store2.get(&"key2".to_string()), Some(&100));
    }
}
