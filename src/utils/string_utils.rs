// src/utils/string_utils.rs

/// Checks if a string is empty or contains only whitespace.
pub fn is_blank(s: &str) -> bool {
    s.trim().is_empty()
}

/// Reverses a string.
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
