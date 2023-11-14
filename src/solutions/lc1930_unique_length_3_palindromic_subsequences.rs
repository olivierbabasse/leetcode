//! <https://leetcode.com/problems/unique-length-3-palindromic-subsequences/>

use std::collections::HashSet;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let mut total = 0;

        for c in 'a'..='z' {
            if let Some(pos1) = s.find(c) {
                if let Some(pos2) = s[pos1 + 1..].rfind(c) {
                    let set = s[pos1 + 1..pos1 + pos2 + 1].chars().collect::<HashSet<_>>();
                    total += set.len();
                }
            }
        }

        total as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::count_palindromic_subsequence("aabca".into()), 3);
        assert_eq!(Solution::count_palindromic_subsequence("adc".into()), 0);
        assert_eq!(Solution::count_palindromic_subsequence("bbcbaba".into()), 4);
    }
}
