//! <https://leetcode.com/problems/count-distinct-numbers-on-board/>

struct Solution {}

/// time-complexity : O(1)
/// space-complexity : O(1)
impl Solution {
    pub fn distinct_integers(n: i32) -> i32 {
        1.max(n - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::distinct_integers(5), 4);
        assert_eq!(Solution::distinct_integers(3), 2);
        assert_eq!(Solution::distinct_integers(1), 1);
    }
}
