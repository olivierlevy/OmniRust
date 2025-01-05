use cached::proc_macro::cached;

/// A simple function with memoization.
/// This caches results of the function for the same inputs.
#[cached]
pub fn fibonacci(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

use cached::SizedCache;
use std::sync::Mutex;

/// A manual cache using `SizedCache`.
pub struct CustomCache {
    cache: Mutex<SizedCache<String, String>>,
}

impl CustomCache {
    /// Creates a new cache with a fixed size.
    pub fn new(size: usize) -> Self {
        Self {
            cache: Mutex::new(SizedCache::with_size(size)),
        }
    }

    /// Gets a cached value or computes it using the provided function.
    pub fn get_or_insert_with<F>(&self, key: &str, compute: F) -> String
    where
        F: Fn() -> String,
    {
        let mut cache = self.cache.lock().unwrap();
        if let Some(value) = cache.get(key) {
            value.clone()
        } else {
            let value = compute();
            cache.cache_set(key.to_string(), value.clone());
            value
        }
    }
}
