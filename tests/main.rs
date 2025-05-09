// tests/main.rs

// This file can be used to declare modules for your tests.
// If you have multiple test files, you can declare them here.

// For example, if you have tests/integration_tests.rs:
mod integration_tests;

// You can also keep simple tests directly in this file if you prefer.
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
