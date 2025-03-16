use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;

#[derive(Serialize, Deserialize)]
pub struct KeyValueStore<K, V>
where
    K: std::cmp::Eq + std::hash::Hash,
{
    pub data: HashMap<K, V>,
    file_path: String,
}

impl<K, V> KeyValueStore<K, V>
where
    K: Serialize + std::cmp::Eq + std::hash::Hash,
    V: Serialize,
{
    pub fn new() -> Self {
        let data = HashMap::<K, V>::new();

        KeyValueStore {
            data,
            file_path: "".to_string(),
        }
    }

    pub fn insert(&mut self, key: K, val: V) -> Option<V> {
        self.data.insert(key, val)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.data.remove(key)
    }

    //pub fn save<E>(&self) -> Result<(), E> {
    //    serde_json::to_string(&self);
    //    Ok(())
    //}
}
