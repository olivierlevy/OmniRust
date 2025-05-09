// sample/tests/main.rs

// This file can be used to declare modules for your sample app's tests.

mod integration_tests;

// You can also keep simple tests directly in this file if you prefer.
#[cfg(test)]
mod tests {
    #[test]
    fn it_works_in_sample() {
        assert_eq!(1 + 1, 2);
    }
}
