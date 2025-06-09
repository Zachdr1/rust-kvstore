use super::kvstore::Backend;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter};
use std::io::{Seek, SeekFrom, Write};
use std::str::FromStr;

fn parse_log_line<K: FromStr>(line: &str) -> Option<(K, String)> {
    let mut parts = line.trim().split('\t');
    let key_str = parts.next()?.to_string();
    let value = parts.next()?.to_string();
    let key = key_str.parse::<K>().ok()?;
    Some((key, value))
}

pub struct SimpleLogBackend<K, V> {
    index: HashMap<K, u64>,
    current_position: u64,
    log_file: File,
    _phantom: std::marker::PhantomData<(K, V)>,
}

impl<K, V> Backend<K, V> for SimpleLogBackend<K, V>
where
    K: Display + std::hash::Hash + std::cmp::Eq + FromStr,
    V: Display + FromStr,
{
    fn new(filepath: &str) -> Self {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(filepath)
            .unwrap();

        let mut index = HashMap::<K, u64>::new();
        let mut current_position = 0u64;

        file.seek(SeekFrom::Start(0)).unwrap();
        let mut reader = BufReader::new(&file);
        let mut line = String::new();
        while reader.read_line(&mut line).unwrap() > 0 {
            let line_bytes = line.len() as u64;

            if let Some((key, value)) = parse_log_line(&line) {
                if value == "__DELETED__" {
                    index.remove(&key);
                } else {
                    index.insert(key, current_position);
                }
            }

            current_position += line_bytes;
            line.clear()
        }

        Self {
            index,
            current_position,
            log_file: file,
            _phantom: std::marker::PhantomData,
        }
    }

    fn insert(&mut self, key: K, val: V) -> Result<Option<V>, std::io::Error> {
        let mut stream = BufWriter::new(&mut self.log_file);
        let entry = format!("{}\t{}\n", key, val);
        let entry_bytes = entry.as_bytes();
        write!(stream, "{}", entry)?;
        stream.flush()?;

        self.index.insert(key, self.current_position);
        self.current_position += entry_bytes.len() as u64;
        Ok(Some(val))
    }

    fn get(&mut self, key: &K) -> Option<V> {
        if let Some(&position) = self.index.get(key) {
            self.log_file.seek(SeekFrom::Start(position)).unwrap();
            let mut reader = BufReader::new(&self.log_file);
            let mut line_str = String::new();
            let _ = reader.read_line(&mut line_str).unwrap();

            let value = line_str.trim().split('\t').nth(1)?.parse::<V>().ok()?;

            Some(value)
        } else {
            None
        }
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        let entry = format!("{}\t__DELETED__\n", key);
        let mut stream = BufWriter::new(&mut self.log_file);
        write!(stream, "{}", entry).ok()?;
        stream.flush().ok()?;

        self.index.remove(key);
        None
    }

    fn flush(&self) -> Result<(), std::io::Error> {
        // Don't need to flush with this implementation
        Ok(())
    }
}
