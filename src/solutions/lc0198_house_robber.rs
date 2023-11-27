//! <https://leetcode.com/problems/house-robber/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    fn irob(index: usize, nums: &Vec<i32>, dp: &mut Vec<Option<i32>>) -> i32 {
        if index >= nums.len() {
            0
        } else if let Some(val) = dp[index] {
            val
        } else {
            let val = i32::max(
                Self::irob(index + 1, nums, dp),
                nums[index] + Self::irob(index + 2, nums, dp),
            );
            dp[index] = Some(val);
            val
        }
    }

    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp = vec![None; nums.len() + 1];
        Self::irob(0, &nums, &mut dp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
    }
}
