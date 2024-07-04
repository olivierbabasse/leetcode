//! <https://leetcode.com/problems/intersection-of-multiple-arrays/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O()
/// space-complexity : O()
impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let len = nums.len();
        let mut freqs = HashMap::<i32, usize>::new();
        nums.into_iter().for_each(|nums| {
            nums.into_iter()
                .for_each(|n| *freqs.entry(n).or_default() += 1)
        });

        let mut res = Vec::new();
        freqs.into_iter().for_each(|(n, count)| {
            if count == len {
                res.push(n);
            }
        });
        res.sort_unstable();
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::array_eq;

    #[test]
    fn test_cases() {
        assert!(array_eq(
            &Solution::intersection(vec![
                vec![3, 1, 2, 4, 5],
                vec![1, 2, 3, 4],
                vec![3, 4, 5, 6]
            ]),
            &[3, 4]
        ));
        assert!(array_eq(
            &Solution::intersection(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            &[]
        ));
    }
}
