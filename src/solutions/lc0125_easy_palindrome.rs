//! <https://leetcode.com/problems/valid-palindrome/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect::<String>();
        let revs = s.chars().rev().collect::<String>();
        s == revs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert!(Solution::is_palindrome(
            "A man, a plan, a canal: Panama".into()
        ));
        assert!(!Solution::is_palindrome("race a car".into()));
    }
}
