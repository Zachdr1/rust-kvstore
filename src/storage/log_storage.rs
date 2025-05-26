use super::kvstore::{Backend, KeyValueStore};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{read_to_string, File, OpenOptions};
use std::io::{BufRead, BufReader};
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

fn parse_log_line(line: &str) -> Option<(String, String)> {
    let mut parts = line.trim().split('\t');
    let key = parts.next()?.to_string();
    let value = parts.next()?.to_string();
    Some((key, value))
}

pub struct SimpleLogBackend<K, V> {
    index: HashMap<String, u64>,
    current_pos: u64,
    log_file: File,
    _phantom: std::marker::PhantomData<(K, V)>,
}

impl<K, V> Backend<K, V> for SimpleLogBackend<K, V> {
    fn new(filepath: &str) -> Self {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(filepath)
            .unwrap();

        let mut index = HashMap::new();
        let mut current_pos = 0u64;

        file.seek(SeekFrom::Start(0)).unwrap();
        let mut reader = BufReader::new(&file);
        let mut line = String::new();
        while reader.read_line(&mut line).unwrap() > 0 {
            let line_bytes = line.len() as u64;

            if let Some((key, _value)) = parse_log_line(&line) {
                index.insert(key, current_pos);
            }

            current_pos += line_bytes;
            line.clear()
        }

        Self {
            index,
            current_pos,
            log_file: file,
            _phantom: std::marker::PhantomData,
        }
    }

    fn insert(&mut self, key: K, val: V) -> Result<Option<V>, std::io::Error> {
        // let res = self.data.insert(key, val);
        // let json = serde_json::to_string(&self.data)?;
        // std::fs::write(&self.filepath, json)?;
        todo!("implement get method")
    }

    fn get(&self, key: &K) -> Option<&V> {
        // self.data.get(key)
        todo!("implement get method")
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        // self.data.remove(key)
        todo!("implement remove method")
    }

    fn flush(&self) -> Result<(), std::io::Error> {
        todo!("implement remove method")
        // let json = serde_json::to_string(&self.data)?;
        // std::fs::write(&self.filepath, json)?;
        // Ok(())
    }
}
