//! <https://leetcode.com/problems/minimum-common-value/>

use std::cmp::Ordering;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (mut i1, mut i2) = (0, 0);
        while i1 < nums1.len() && i2 < nums2.len() {
            match nums1[i1].cmp(&nums2[i2]) {
                Ordering::Greater => i2 += 1,
                Ordering::Less => i1 += 1,
                Ordering::Equal => return nums1[i1],
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::get_common(vec![1, 2, 3], vec![2, 4]), 2);
        assert_eq!(Solution::get_common(vec![1, 2, 3, 6], vec![2, 3, 4, 5]), 2);
    }
}
