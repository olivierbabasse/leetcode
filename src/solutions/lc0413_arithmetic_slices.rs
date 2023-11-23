//! <https://leetcode.com/problems/arithmetic-slices/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut cur = 0;
        let mut total = 0;
        for i in 2..n {
            if nums[i] - nums[i - 1] == nums[i - 1] - nums[i - 2] {
                cur += 1;
                total += cur;
            } else {
                cur = 0;
            }
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::number_of_arithmetic_slices(vec![1, 2, 3, 4]), 3);
        assert_eq!(Solution::number_of_arithmetic_slices(vec![1]), 0);
    }
}
