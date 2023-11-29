//! <https://leetcode.com/problems/a-number-after-a-double-reversal/>

struct Solution {}

/// time-complexity : O(1)
/// space-complexity : O(1)
impl Solution {
    pub fn is_same_after_reversals(num: i32) -> bool {
        num == 0 || num % 10 != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert!(Solution::is_same_after_reversals(526));
        assert!(!Solution::is_same_after_reversals(1800));
        assert!(Solution::is_same_after_reversals(0));
    }
}
