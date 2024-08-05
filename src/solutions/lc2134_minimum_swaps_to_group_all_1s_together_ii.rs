//! <https://leetcode.com/problems/minimum-swaps-to-group-all-1s-together-ii/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let ones: i32 = nums.iter().sum();
        let mut count = nums[0];
        let mut min_swaps = i32::MAX;
        let mut end = 0;
        for start in 0..nums.len() {
            if start > 0 {
                count -= nums[start - 1];
            }

            while end - start + 1 < ones as usize {
                end += 1;
                count += nums[end % nums.len()];
            }

            min_swaps = min_swaps.min(ones - count);
        }
        min_swaps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::min_swaps(vec![0, 1, 0, 1, 1, 0, 0]), 1);
        assert_eq!(Solution::min_swaps(vec![0, 1, 1, 1, 0, 0, 1, 1, 0]), 2);
        assert_eq!(Solution::min_swaps(vec![1, 1, 0, 0, 1]), 0);
    }
}
