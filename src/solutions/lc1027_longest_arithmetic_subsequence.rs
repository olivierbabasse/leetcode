//! <https://leetcode.com/problems/longest-arithmetic-subsequence/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(n^2)
/// space-complexity : O(n^2)
impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        let mut diffs = HashMap::new();
        let mut maxlen = 0;
        for index in 0..nums.len() {
            for i in 0..index {
                let diff = nums[index] - nums[i];
                let len = 1 + *diffs.entry((i, diff)).or_insert(1);
                diffs.insert((index, diff), len);
                maxlen = maxlen.max(len);
            }
        }
        maxlen
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::longest_arith_seq_length(vec![3, 6, 9, 12]), 4);
        assert_eq!(Solution::longest_arith_seq_length(vec![9, 4, 7, 2, 10]), 3);
        assert_eq!(
            Solution::longest_arith_seq_length(vec![20, 1, 15, 3, 10, 5, 8]),
            4
        );
    }
}
