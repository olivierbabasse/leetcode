//! <https://leetcode.com/problems/is-subsequence/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let (mut si, mut ti) = (0, 0);
        while si < s.len() && ti < t.len() {
            if s.as_bytes()[si] == t.as_bytes()[ti] {
                si += 1;
            }
            ti += 1;
        }
        si == s.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert!(Solution::is_subsequence("abc".into(), "ahbgdc".into()));
        assert!(!Solution::is_subsequence("axc".into(), "ahbgdc".into()));
        assert!(Solution::is_subsequence("".into(), "ahbgdc".into()));
    }
}
