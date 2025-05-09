// sample/tests/main.rs

// This file is compiled as a separate test crate.
// It can contain its own tests.
// Other .rs files in the tests/ directory (like integration_tests.rs)
// will also be compiled as separate test crates automatically by Cargo.

#[cfg(test)]
mod main_specific_tests { // Renamed to avoid conflict if integration_tests.rs also has a 'tests' mod
    // If these tests need to use items from omnirust_sample_app,
    // they would also need `use omnirust_sample_app::some_module;`
    #[test]
    fn it_works_in_sample_main_test_file() {
        assert_eq!(1 + 1, 2);
    }
}
