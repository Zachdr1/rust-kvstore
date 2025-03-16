use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct KeyValueStore<K, V>
where
    K: std::cmp::Eq + std::hash::Hash,
{
    data: HashMap<K, V>,
    pub file_path: String,
}

impl<K, V> KeyValueStore<K, V>
where
    K: for<'a> Deserialize<'a> + Serialize + std::cmp::Eq + std::hash::Hash + Clone,
    V: for<'a> Deserialize<'a> + Serialize + Clone,
{
    pub fn new(file_path: &str) -> Self {
        if !Path::new(file_path).exists() {
            let _ = File::create(file_path).unwrap();
        }

        let data = HashMap::<K, V>::new();

        Self {
            data,
            file_path: file_path.to_string(),
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

    pub fn save(&self) -> Result<(), std::io::Error> {
        let json = serde_json::to_string(&self)?;
        std::fs::write(&self.file_path, json)?;
        Ok(())
    }

    pub fn load(self) -> Result<Self, std::io::Error> {
        let file_content = read_to_string(&self.file_path)?;
        if !file_content.is_empty() {
            println!("Data found in {}, loading..", &self.file_path);
            let store: Self = serde_json::from_str(&file_content)?;
            Ok(store)
        } else {
            println!("{} empty", &self.file_path);
            Ok(Self {
                file_path: self.file_path,
                data: self.data,
            })
        }
    }
}
