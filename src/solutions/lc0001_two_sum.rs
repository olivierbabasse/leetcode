//! <https://leetcode.com/problems/two-sum/>
use std::collections::HashMap;

struct Solution {}

/// one loop on nums, looking for complement in a HashMap, storing values along the way
/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut already_seen = HashMap::new();
        for (index, num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(complement_index) = already_seen.get(&complement) {
                return vec![index as i32, *complement_index];
            }
            already_seen.insert(num, index as i32);
        }
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::utils::array_eq;

    #[test]
    fn test_cases() {
        assert!(array_eq(&Solution::two_sum(vec![2, 7, 11, 15], 9), &[0, 1]));
        assert!(array_eq(&Solution::two_sum(vec![3, 2, 4], 6), &[1, 2]));
        assert!(array_eq(&Solution::two_sum(vec![3, 3], 6), &[0, 1]));
    }
}
