use cached::SizedCache;

pub struct Cache {
    cache: SizedCache<String, String>,
}

impl Cache {
    pub fn new(capacity: usize) -> Self {
        Cache {
            cache: SizedCache::with_capacity(capacity),
        }
    }

    pub fn get(&mut self, key: &str) -> Option<&String> {
        self.cache.get(key)
    }

    pub fn set(&mut self, key: String, value: String) {
        self.cache.set(key, value);
    }

    pub fn invalidate(&mut self, key: &str) {
        self.cache.invalidate(key);
    }
}
