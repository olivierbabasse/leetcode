//! <https://leetcode.com/problems/minimize-the-maximum-difference-of-pairs/>

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(1)
impl Solution {
    fn valid_pairs(nums: &[i32], threshold: i32) -> usize {
        let (mut index, mut count) = (0, 0);
        while index < nums.len() - 1 {
            if nums[index + 1] - nums[index] <= threshold {
                count += 1;
                index += 1;
            }
            index += 1;
        }
        count
    }

    pub fn minimize_max(mut nums: Vec<i32>, p: i32) -> i32 {
        let len = nums.len();
        nums.sort_unstable();
        let (mut left, mut right) = (0, nums[len - 1] - nums[0]);
        while left < right {
            let mid = (left + right) / 2;
            if Self::valid_pairs(&nums, mid) >= p as usize {
                right = mid;
            } else {
                left = mid + 1
            }
        }
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::minimize_max(vec![10, 1, 2, 7, 1, 3], 2), 1);
        assert_eq!(Solution::minimize_max(vec![4, 2, 1, 2], 1), 0);
    }
}
