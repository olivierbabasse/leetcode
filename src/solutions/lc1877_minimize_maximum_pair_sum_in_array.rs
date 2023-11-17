//! <https://leetcode.com/problems/minimize-maximum-pair-sum-in-array/>

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(1)
impl Solution {
    pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        nums.sort_unstable();
        let mut max_pair_sum = 0;
        for i in 0..n {
            max_pair_sum = max_pair_sum.max(nums[i] + nums[n - i - 1]);
        }
        max_pair_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::min_pair_sum(vec![3, 5, 2, 3]), 7);
        assert_eq!(Solution::min_pair_sum(vec![3, 5, 4, 2, 4, 6]), 8);
    }
}
