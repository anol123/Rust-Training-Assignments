use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct CacheEntry {
    pub value: Vec<u8>,
    pub expires_at: Instant,
}

pub struct InMemoryCache {
    store: Arc<Mutex<HashMap<String, CacheEntry>>>,
    cleanup_thread: Option<thread::JoinHandle<()>>,
    shutdown_flag: Arc<Mutex<bool>>,
}

impl InMemoryCache {
    pub fn new(cleanup_interval: Duration) -> Self {
        let store: Arc<Mutex<HashMap<String, CacheEntry>>> = Arc::new(Mutex::new(HashMap::new()));
        let shutdown_flag = Arc::new(Mutex::new(false));

        let store_clone = Arc::clone(&store);
        let shutdown_clone = Arc::clone(&shutdown_flag);

        let handle = thread::spawn(move || {
            loop {
                thread::sleep(cleanup_interval);
                if *shutdown_clone.lock().unwrap() {
                    break;
                }

                let mut store = store_clone.lock().unwrap();
                let now = Instant::now();
                store.retain(|_, entry| entry.expires_at > now);
            }
        });

        InMemoryCache {
            store,
            cleanup_thread: Some(handle),
            shutdown_flag,
        }
    }

    pub fn insert(&self, key: String, value: Vec<u8>, ttl: Duration) {
        let expires_at = Instant::now() + ttl;
        let entry = CacheEntry { value, expires_at };

        let mut store = self.store.lock().unwrap();
        store.insert(key, entry);
    }

    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        let store = self.store.lock().unwrap();
        store.get(key).and_then(|entry| {
            if Instant::now() < entry.expires_at {
                Some(entry.value.clone()) // ✅ clone to avoid reference issues
            } else {
                None
            }
        })
    }
}

impl Drop for InMemoryCache {
    fn drop(&mut self) {
        println!("Shutting down cache…");

        *self.shutdown_flag.lock().unwrap() = true;

        if let Some(handle) = self.cleanup_thread.take() {
            handle.join().unwrap();
        }

        println!("Cleanup completed. Cache shut down.");
    }
}
