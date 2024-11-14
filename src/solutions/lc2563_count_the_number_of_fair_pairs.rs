//! <https://leetcode.com/problems/count-the-number-of-fair-pairs/>

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(1)
impl Solution {
    fn count(nums: &[i32], upper: i32) -> i64 {
        let (mut l, mut r) = (0, nums.len() - 1);
        let mut count = 0;
        while l < r {
            if nums[l] + nums[r] <= upper {
                count += r - l;
                l += 1;
            } else {
                r -= 1;
            }
        }
        count as i64
    }

    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        nums.sort_unstable();
        Self::count(&nums, upper) - Self::count(&nums, lower - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::count_fair_pairs(vec![0, 1, 7, 4, 4, 5], 3, 6), 6);
        assert_eq!(Solution::count_fair_pairs(vec![1, 7, 9, 2, 5], 11, 11), 1);
    }
}
