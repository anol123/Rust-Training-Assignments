mod cache;

use std::thread::sleep;
use std::time::Duration;
use crate::cache::InMemoryCache;

fn main() {
    let cache = InMemoryCache::new(Duration::from_secs(1));
    cache.insert("greeting".to_string(), b"hello world".to_vec(), Duration::from_secs(3));

    for i in 0..5 {
        println!("Checking at {}s:", i);
        if let Some(val) = cache.get("greeting") {
            println!("Found: {}", std::str::from_utf8(&val).unwrap());
        } else {
            println!("Entry expired or missing.");
        }
        std::thread::sleep(Duration::from_secs(1));
    }
}