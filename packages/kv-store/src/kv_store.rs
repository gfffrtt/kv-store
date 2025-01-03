use std::collections::HashMap;

use tokio::sync::Mutex;

pub struct KvStore {
    pub store: Mutex<HashMap<String, String>>,
}

impl KvStore {
    pub fn new() -> Self {
        KvStore {
            store: Mutex::new(HashMap::new()),
        }
    }

    pub async fn get(&self, key: String) -> Option<String> {
        let store = self.store.lock().await;
        store.get(&key).cloned()
    }

    pub async fn set(&self, key: String, value: String) {
        let mut store = self.store.lock().await;
        store.insert(key, value);
    }

    pub async fn delete(&self, key: String) -> Option<String> {
        let mut store = self.store.lock().await;
        store.remove(&key).clone()
    }

    pub async fn exists(&self, key: String) -> bool {
        let store = self.store.lock().await;
        store.get(&key).is_some()
    }

    pub async fn keys(&self) -> Vec<String> {
        let store = self.store.lock().await;
        store.keys().cloned().collect()
    }
}
