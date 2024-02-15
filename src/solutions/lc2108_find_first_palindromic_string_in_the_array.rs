//! <https://leetcode.com/problems/find-first-palindromic-string-in-the-array/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        words
            .into_iter()
            .find(|s| s == &s.chars().rev().collect::<String>())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::first_palindrome(vec![
                "abc".into(),
                "car".into(),
                "ada".into(),
                "racecar".into(),
                "cool".into()
            ]),
            "ada".to_string()
        );
        assert_eq!(
            Solution::first_palindrome(vec!["notapalindrome".into(), "racecar".into()]),
            "racecar".to_string()
        );
        assert_eq!(
            Solution::first_palindrome(vec!["def".into(), "ghi".into()]),
            "".to_string()
        );
    }
}
