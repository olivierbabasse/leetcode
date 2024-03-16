//! <https://leetcode.com/problems/intersection-of-two-arrays/>

use std::collections::HashSet;

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let s1: HashSet<i32> = nums1.into_iter().collect();
        let s2: HashSet<i32> = nums2.into_iter().collect();
        s1.intersection(&s2).copied().collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]),
            vec![2]
        );
        assert!(utils::array_eq(
            &Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]),
            &[4, 9]
        ));
    }
}
