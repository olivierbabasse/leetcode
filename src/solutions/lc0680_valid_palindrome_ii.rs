//! <https://leetcode.com/problems/valid-palindrome-ii/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    fn is_valid(s: &str, mut begin: usize, mut end: usize, can_skip_1: bool) -> bool {
        while begin <= end {
            if s.as_bytes()[begin] != s.as_bytes()[end] {
                if can_skip_1 {
                    return Self::is_valid(s, begin + 1, end, false)
                        || Self::is_valid(s, begin, end - 1, false);
                } else {
                    return false;
                }
            }
            begin += 1;
            end -= 1;
        }
        true
    }

    pub fn valid_palindrome(s: String) -> bool {
        Self::is_valid(&s, 0, s.len() - 1, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert!(Solution::valid_palindrome("aba".into()));
        assert!(Solution::valid_palindrome("abca".into()));
        assert!(!Solution::valid_palindrome("abc".into()));
    }
}
