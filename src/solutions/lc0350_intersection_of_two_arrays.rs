//! <https://leetcode.com/problems/intersection-of-two-arrays-ii/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(n+m)
/// space-complexity : O(n+m)

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut freqs = HashMap::<i32, usize>::new();
        nums1
            .into_iter()
            .for_each(|n| *freqs.entry(n).or_default() += 1);
        nums2.into_iter().for_each(|n| {
            if let Some(f) = freqs.get_mut(&n) {
                if *f > 0 {
                    res.push(n);
                    *f -= 1;
                }
            }
        });
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]),
            vec![2, 2]
        );
        assert_eq!(
            Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]),
            vec![9, 4]
        );
    }
}
