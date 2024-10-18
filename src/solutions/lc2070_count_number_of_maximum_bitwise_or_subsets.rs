//! <https://leetcode.com/problems/count-number-of-maximum-bitwise-or-subsets/>

struct Solution {}

/// time-complexity : O(2^n)
/// space-complexity : O(n)
impl Solution {
    fn rec(nums: &[i32], index: usize, cur: i32, target: i32) -> i32 {
        if index == nums.len() {
            return if cur == target { 1 } else { 0 };
        }

        Self::rec(nums, index + 1, cur, target)
            + Self::rec(nums, index + 1, cur | nums[index], target)
    }

    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let max = nums.iter().fold(0, |acc, x| acc | x);

        Self::rec(&nums, 0, 0, max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::count_max_or_subsets(vec![3, 1]), 2);
        assert_eq!(Solution::count_max_or_subsets(vec![2, 2, 2]), 7);
        assert_eq!(Solution::count_max_or_subsets(vec![3, 2, 1, 5]), 6);
    }
}
