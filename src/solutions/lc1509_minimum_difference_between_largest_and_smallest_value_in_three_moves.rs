//! <https://leetcode.com/problems/minimum-difference-between-largest-and-smallest-value-in-three-moves/>

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(1)
impl Solution {
    pub fn min_difference(mut nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len <= 4 {
            return 0;
        }

        nums.sort_unstable();

        let (mut left, mut right) = (0, len - 4);
        let mut min = i32::MAX;
        while left < 4 {
            min = min.min(nums[right] - nums[left]);
            left += 1;
            right += 1;
        }
        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::min_difference(vec![5, 3, 2, 4]), 0);
        assert_eq!(Solution::min_difference(vec![1, 5, 0, 10, 14]), 1);
        assert_eq!(Solution::min_difference(vec![3, 100, 20]), 0);
    }
}
