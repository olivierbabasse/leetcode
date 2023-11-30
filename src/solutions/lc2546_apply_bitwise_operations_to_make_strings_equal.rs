//! <https://leetcode.com/problems/apply-bitwise-operations-to-make-strings-equal/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn make_strings_equal(s: String, target: String) -> bool {
        s.chars().any(|c| c == '1') == target.chars().any(|c| c == '1')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert!(Solution::make_strings_equal("1010".into(), "0110".into()));
        assert!(!Solution::make_strings_equal("11".into(), "00".into()));
    }
}
