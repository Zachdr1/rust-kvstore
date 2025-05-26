pub trait Backend<K, V> {
    fn new(filepath: &str) -> Self;
    fn insert(&mut self, key: K, val: V) -> Result<Option<V>, std::io::Error>;
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
    fn flush(&self) -> Result<(), std::io::Error>;
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

    pub fn insert(&mut self, key: K, val: V) -> Result<Option<V>, std::io::Error> {
        self.backend.insert(key, val)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.backend.get(key)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.backend.remove(key)
    }

    pub fn flush(&mut self) -> Result<(), std::io::Error> {
        self.backend.flush()
    }
}
