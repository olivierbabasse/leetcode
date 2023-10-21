//! <https://leetcode.com/problems/constrained-subsequence-sum/>

use std::collections::VecDeque;

struct Solution {}

/// time-complexity : O(n*k)
/// space-complexity : O(k)
impl Solution {
    pub fn constrained_subset_sum(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut max = nums[0];
        let mut max_deque = VecDeque::new();

        for i in 0..nums.len() {
            if let Some(f) = max_deque.front() {
                nums[i] += f;
            }
            max = max.max(nums[i]);

            while !max_deque.is_empty() && nums[i] > *max_deque.back().unwrap() {
                max_deque.pop_back();
            }

            if nums[i] > 0 {
                max_deque.push_back(nums[i]);
            }

            if i >= k as usize
                && !max_deque.is_empty()
                && *max_deque.front().unwrap() == nums[i - k as usize]
            {
                max_deque.pop_front();
            }
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::constrained_subset_sum(vec![10, 2, -10, 5, 20], 2),
            37
        );
        assert_eq!(Solution::constrained_subset_sum(vec![-1, -2, -3], 1), -1);
        assert_eq!(
            Solution::constrained_subset_sum(vec![10, -2, -10, -5, 20], 2),
            23
        );
    }
}
