use omnirust::utils::caching::{fibonacci, CustomCache};

#[test]
fn test_cached_fibonacci() {
    assert_eq!(fibonacci(10), 55);
    assert_eq!(fibonacci(20), 6765); // Tests cached result
}

#[test]
fn test_custom_cache() {
    let cache = CustomCache::new(2);

    let value = cache.get_or_insert_with("key1", || "value1".to_string());
    assert_eq!(value, "value1");

    let cached_value = cache.get_or_insert_with("key1", || "shouldn't run".to_string());
    assert_eq!(cached_value, "value1"); // Verifies cached result
}
