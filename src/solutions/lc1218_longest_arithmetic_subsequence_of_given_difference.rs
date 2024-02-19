//! <https://leetcode.com/problems/longest-arithmetic-subsequence-of-given-difference/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
        let mut lengths = HashMap::new();
        let mut max = 1;
        for a in arr {
            if let Some(&val) = lengths.get(&(a - difference)) {
                lengths.insert(a, val + 1);
                max = max.max(val + 1);
            } else {
                lengths.insert(a, 1);
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::longest_subsequence(vec![1, 2, 3, 4], 1), 4);
        assert_eq!(Solution::longest_subsequence(vec![1, 3, 5, 7], 1), 1);
        assert_eq!(
            Solution::longest_subsequence(vec![1, 5, 7, 8, 5, 3, 4, 2, 1], -2),
            4
        );
    }
}
