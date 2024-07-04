//! <https://leetcode.com/problems/find-the-difference-of-two-arrays/>

use std::collections::HashSet;

struct Solution {}

/// time-complexity : O(n+m)
/// space-complexity : O(n+m)
impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let f1: HashSet<_> = nums1.into_iter().collect();
        let f2: HashSet<_> = nums2.into_iter().collect();

        let mut res1 = Vec::new();
        f1.iter().for_each(|n| {
            if !f2.contains(n) {
                res1.push(*n)
            }
        });

        let mut res2 = Vec::new();
        f2.iter().for_each(|n| {
            if !f1.contains(n) {
                res2.push(*n)
            }
        });

        vec![res1, res2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::array_of_arrays_eq;

    #[test]
    fn test_cases() {
        assert!(array_of_arrays_eq(
            &Solution::find_difference(vec![1, 2, 3], vec![2, 4, 6]),
            &[vec![3, 1], vec![4, 6]]
        ));
        assert!(array_of_arrays_eq(
            &Solution::find_difference(vec![1, 2, 3, 3], vec![1, 1, 2, 2]),
            &[vec![3], vec![]]
        ));
    }
}
