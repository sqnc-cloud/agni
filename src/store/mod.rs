mod entry;

pub use entry::Entry;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct Store {
    data: Arc<RwLock<HashMap<String, Entry>>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn set(&self, key: String, value: Vec<u8>) {
        let mut data = self.data.write().unwrap();
        data.insert(key.clone(), Entry::new(key, value));
    }

    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        let data = self.data.read().unwrap();
        data.get(key).map(|e| e.value.clone())
    }

    pub fn delete(&self, key: &str) -> bool {
        let mut data = self.data.write().unwrap();
        data.remove(key).is_some()
    }
}
