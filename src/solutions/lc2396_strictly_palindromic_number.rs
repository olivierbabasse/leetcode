//! <https://leetcode.com/problems/strictly-palindromic-number/>

struct Solution {}

/// time-complexity : O(1)
/// space-complexity : O(1)
impl Solution {
    pub fn is_strictly_palindromic(_n: i32) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert!(!Solution::is_strictly_palindromic(9));
        assert!(!Solution::is_strictly_palindromic(4));
    }
}
