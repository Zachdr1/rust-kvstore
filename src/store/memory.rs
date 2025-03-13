use std::collections::HashMap;

pub struct KeyValueStore<K, V> {
    pub data: HashMap<K, V>,
    file_path: String,
}

impl<K, V> KeyValueStore<K, V> {
    pub fn new() -> Self {
        let data = HashMap::<K, V>::new();

        KeyValueStore {
            data,
            file_path: "".to_string(),
        }
    }
}
