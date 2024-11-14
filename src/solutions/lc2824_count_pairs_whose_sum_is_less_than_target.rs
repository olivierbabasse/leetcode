//! <https://leetcode.com/problems/count-pairs-whose-sum-is-less-than-target/>

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(1)
impl Solution {
    pub fn count_pairs(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let (mut l, mut r) = (0, nums.len() - 1);
        let mut count = 0;
        while l < r {
            if nums[l] + nums[r] < target {
                count += r - l;
                l += 1;
            } else {
                r -= 1;
            }
        }
        count as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::count_pairs(vec![-1, 1, 2, 3, 1], 2), 3);
        assert_eq!(Solution::count_pairs(vec![-6, 2, 5, -2, -7, -1, 3], -2), 10);
    }
}
