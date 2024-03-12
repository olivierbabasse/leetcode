//! <https://leetcode.com/problems/number-of-good-pairs/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut counts = HashMap::new();
        let mut count = 0;

        nums.iter().for_each(|n| {
            let c = counts.entry(n).or_insert(0);
            count += *c;
            *c += 1;
        });

        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4);
        assert_eq!(Solution::num_identical_pairs(vec![1, 1, 1, 1]), 6);
        assert_eq!(Solution::num_identical_pairs(vec![1, 2, 3]), 0);
    }
}
