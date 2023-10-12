//! <https://leetcode.com/problems/majority-element/>

use std::collections::HashMap;

struct Solution1 {}

impl Solution1 {
    /// naive implementation
    /// time-complexity : O(n)
    /// space-complexity : O(n)
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut counts = HashMap::new();

        nums.into_iter().for_each(|n| {
            *counts.entry(n).or_insert(0) += 1;
        });

        counts.into_iter().max_by_key(|(_, v)| *v).unwrap().0
    }
}

struct Solution2 {}

impl Solution2 {
    /// using Boyer-Moore majority voting algorithm
    /// time-complexity : O(n)
    /// space-complexity : O(1)
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut votes = 0;
        let mut candidate = -1;

        for num in nums {
            if votes == 0 {
                candidate = num;
                votes = 1;
            } else if num == candidate {
                votes += 1;
            } else {
                votes -= 1;
            }
        }

        // not doing check pass since we known there is a majority element

        candidate
    }
}

#[cfg(test)]
mod tests {
    use super::{Solution1, Solution2};

    #[test]
    fn test_cases_1() {
        assert_eq!(Solution1::majority_element(vec![3, 2, 3]), 3);
        assert_eq!(Solution1::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }

    #[test]
    fn test_cases_2() {
        assert_eq!(Solution2::majority_element(vec![3, 2, 3]), 3);
        assert_eq!(Solution2::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}
