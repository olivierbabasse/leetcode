//! <https://leetcode.com/problems/number-of-good-pairs/>

use std::collections::HashMap;

struct Solution1 {}

impl Solution1 {
    /// time-complexity : O(n^2)
    /// space-complexity : O(1)
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] == nums[j] {
                    count += 1;
                }
            }
        }
        count
    }
}

struct Solution2 {}

impl Solution2 {
    /// time-complexity : O(n)
    /// space-complexity : O(n)
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
    use super::{Solution1, Solution2};

    #[test]
    fn test_cases_1() {
        assert_eq!(Solution1::num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4);
        assert_eq!(Solution1::num_identical_pairs(vec![1, 1, 1, 1]), 6);
        assert_eq!(Solution1::num_identical_pairs(vec![1, 2, 3]), 0);
    }

    #[test]
    fn test_cases_2() {
        assert_eq!(Solution2::num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4);
        assert_eq!(Solution2::num_identical_pairs(vec![1, 1, 1, 1]), 6);
        assert_eq!(Solution2::num_identical_pairs(vec![1, 2, 3]), 0);
    }
}
