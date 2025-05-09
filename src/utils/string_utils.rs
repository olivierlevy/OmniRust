// src/utils/string_utils.rs

//! # String Utilities
//!
//! This module provides common utility functions for string manipulation.
//! These functions are designed to be simple, efficient, and widely applicable.

/// Checks if a string is empty or contains only whitespace.
///
/// A string is considered blank if, after trimming leading and trailing whitespace,
/// it is empty.
///
/// # Examples
///
/// ```
/// use omnirust::utils::string_utils::is_blank; // Adjust path based on your project structure
///
/// assert!(is_blank(""));
/// assert!(is_blank("   "));
/// assert!(is_blank("\t\n\r "));
/// assert!(!is_blank("hello"));
/// assert!(!is_blank("  hello  "));
/// ```
pub fn is_blank(s: &str) -> bool {
    s.trim().is_empty()
}

/// Reverses a string.
///
/// This function takes a string slice and returns a new `String`
/// with its characters in reverse order. It correctly handles Unicode characters.
///
/// # Examples
///
/// ```
/// use omnirust::utils::string_utils::reverse; // Adjust path
///
/// assert_eq!(reverse("hello"), "olleh");
/// assert_eq!(reverse("rust"), "tsur");
/// assert_eq!(reverse("你好"), "好你"); // Unicode example
/// assert_eq!(reverse(""), "");
/// ```
pub fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_blank() {
        assert!(is_blank(""));
        assert!(is_blank("   "));
        assert!(!is_blank("hello"));
        assert!(!is_blank("  hello  "));
    }

    #[test]
    fn test_reverse() {
        assert_eq!(reverse("hello"), "olleh");
        assert_eq!(reverse(""), "");
        assert_eq!(reverse("rust"), "tsur");
    }
}
