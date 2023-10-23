//! <https://leetcode.com/problems/max-dot-product-of-two-subsequences/>

use std::collections::HashMap;

/// time-complexity : O(len1*len2)
/// space-complexity : O(len1*len2)
struct Solution {}

impl Solution {
    fn max_subsequence(
        index1: usize,
        index2: usize,
        nums1: &Vec<i32>,
        nums2: &Vec<i32>,
        memo: &mut HashMap<(usize, usize), Option<i32>>,
    ) -> Option<i32> {
        if let Some(max) = memo.get(&(index1, index2)) {
            return *max;
        }
        if index1 >= nums1.len() || index2 >= nums2.len() {
            memo.insert((index1, index2), None);
            return None;
        }

        let n = nums1[index1] * nums2[index2];
        let mut sums = Vec::new();
        sums.push(n);
        if let Some(max) = Self::max_subsequence(index1 + 1, index2 + 1, nums1, nums2, memo) {
            sums.push(n + max);
            sums.push(max);
        }
        if let Some(max) = Self::max_subsequence(index1 + 1, index2, nums1, nums2, memo) {
            sums.push(max);
        }
        if let Some(max) = Self::max_subsequence(index1, index2 + 1, nums1, nums2, memo) {
            sums.push(max);
        }

        let max = sums.iter().max().cloned();
        memo.insert((index1, index2), max);
        max
    }

    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut memo = HashMap::new();
        Self::max_subsequence(0, 0, &nums1, &nums2, &mut memo).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::max_dot_product(vec![2, 1, -2, 5], vec![3, 0, -6]),
            18
        );
        assert_eq!(Solution::max_dot_product(vec![3, -2], vec![2, -6, 7]), 21);
        assert_eq!(Solution::max_dot_product(vec![-1, -1], vec![1, 1]), -1);
        assert_eq!(
            Solution::max_dot_product(vec![-5, -1, -2], vec![3, 3, 5, 5]),
            -3
        );
        assert_eq!(
            Solution::max_dot_product(vec![5, -4, -3], vec![-4, -3, 0, -4, 2]),
            28
        );
    }
}
