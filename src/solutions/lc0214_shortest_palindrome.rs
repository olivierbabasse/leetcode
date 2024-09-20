//! <https://leetcode.com/problems/shortest-palindrome/>

struct Solution {}

/// time-complexity : O(n^2)
/// space-complexity : O(n)
impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        let len = s.len();
        let rev = s.chars().rev().collect::<String>();
        for i in 0..len {
            if s[0..len - i] == rev[i..] {
                return rev[0..i].to_string() + s.as_str();
            }
        }
        rev + s.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::shortest_palindrome("aacecaaa".into()),
            "aaacecaaa".to_string()
        );
        assert_eq!(
            Solution::shortest_palindrome("abcd".into()),
            "dcbabcd".to_string()
        );
    }
}
