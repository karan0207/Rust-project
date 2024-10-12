use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use chrono::{DateTime, Utc, Duration};
use tokio::time::{sleep, Duration as TokioDuration};

#[derive(Clone)]
struct CacheItem<T> {
    value: T,
    expiry: Option<DateTime<Utc>>,  // Time to live (TTL)
}

struct HyperCache<K, V> {
    cache: Arc<RwLock<HashMap<K, CacheItem<V>>>>,
    eviction_policy: EvictionPolicy,
    max_size: usize,
}

#[derive(Clone)]
enum EvictionPolicy {
    LRU,
    FIFO,
}

impl<K, V> HyperCache<K, V>
where
    K: std::cmp::Eq + std::hash::Hash + Clone + std::fmt::Debug,
    V: Clone,
{
    pub fn new(eviction_policy: EvictionPolicy, max_size: usize) -> Self {
        HyperCache {
            cache: Arc::new(RwLock::new(HashMap::new())),
            eviction_policy,
            max_size,
        }
    }

    pub fn insert(&self, key: K, value: V, ttl_secs: Option<i64>) {
        let expiry = ttl_secs.map(|secs| Utc::now() + Duration::seconds(secs));
        let new_item = CacheItem { value, expiry };

        let mut cache = self.cache.write().unwrap();
        cache.insert(key.clone(), new_item);
        
        if cache.len() > self.max_size {
            self.evict(&key);
        }
    }

    pub fn get(&self, key: &K) -> Option<V> {
        let mut cache = self.cache.write().unwrap();
        if let Some(item) = cache.get(key) {
            if let Some(expiry) = item.expiry {
                if Utc::now() > expiry {
                    // Item has expired, remove it
                    cache.remove(key);
                    return None;
                }
            }
            return Some(item.value.clone());
        }
        None
    }

    fn evict(&self, _key: &K) {
        let mut cache = self.cache.write().unwrap();

        // Eviction policies: LRU, FIFO
        match self.eviction_policy {
            EvictionPolicy::LRU => {
                // For simplicity, we just remove the first inserted (you can implement advanced LRU tracking)
                let first_key = cache.keys().next().cloned();
                if let Some(key) = first_key {
                    println!("Evicting key: {:?}", key);
                    cache.remove(&key);
                }
            }
            EvictionPolicy::FIFO => {
                // Remove based on insertion order (could track separately if needed)
                let first_key = cache.keys().next().cloned();
                if let Some(key) = first_key {
                    println!("Evicting key: {:?}", key);
                    cache.remove(&key);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let cache = HyperCache::new(EvictionPolicy::LRU, 3);

    cache.insert("a", 1, Some(5));  // Insert with TTL 5 seconds
    cache.insert("b", 2, None);     // Insert without TTL
    cache.insert("c", 3, Some(10)); // Insert with TTL 10 seconds

    // Fetch values
    println!("Get a: {:?}", cache.get(&"a"));
    println!("Get b: {:?}", cache.get(&"b"));

    sleep(TokioDuration::from_secs(6)).await;  // Simulate waiting

    // Check after expiry
    println!("Get a after expiry: {:?}", cache.get(&"a"));
    println!("Get c: {:?}", cache.get(&"c"));

    cache.insert("d", 4, None); // Insert new element, triggers eviction due to max_size

    println!("Get b: {:?}", cache.get(&"b"));
    println!("Get d: {:?}", cache.get(&"d"));
}
