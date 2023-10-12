//! <https://leetcode.com/problems/two-sum/>
use std::collections::HashMap;

struct Solution1 {}

impl Solution1 {
    /// naive implementation : two nested loops on nums
    /// time-complexity : O(n^2)
    /// space-complexity : O(n)
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        Vec::new()
    }
}

struct Solution2 {}

impl Solution2 {
    /// one loop on nums, looking for complement in a HashMap, storing values along the way
    /// time-complexity : O(n)
    /// space-complexity : O(n)
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut already_seen = HashMap::new();
        for (index, num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(other_index) = already_seen.get(&complement) {
                return vec![index as i32, *other_index];
            }
            already_seen.insert(num, index as i32);
        }
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{Solution1, Solution2};
    use crate::utils::array_eq;

    #[test]
    fn test_cases_1() {
        assert!(array_eq(
            &Solution1::two_sum(vec![2, 7, 11, 15], 9),
            &[0, 1]
        ));
        assert!(array_eq(&Solution1::two_sum(vec![3, 2, 4], 6), &[1, 2]));
        assert!(array_eq(&Solution1::two_sum(vec![3, 3], 6), &[0, 1]));
    }

    #[test]
    fn test_cases_2() {
        assert!(array_eq(
            &Solution2::two_sum(vec![2, 7, 11, 15], 9),
            &[0, 1]
        ));
        assert!(array_eq(&Solution2::two_sum(vec![3, 2, 4], 6), &[1, 2]));
        assert!(array_eq(&Solution2::two_sum(vec![3, 3], 6), &[0, 1]));
    }
}
