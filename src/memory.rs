use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};

pub struct KeyValueStore<K, V> {
    data: HashMap<K, V>,
    file_path: String,
}
