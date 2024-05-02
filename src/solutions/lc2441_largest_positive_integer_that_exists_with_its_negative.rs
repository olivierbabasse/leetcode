//! <https://leetcode.com/problems/largest-positive-integer-that-exists-with-its-negative/>

use std::collections::HashSet;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let mut max = -1;
        let hnums = nums.iter().collect::<HashSet<_>>();
        for &n in &nums {
            if n.abs() > max && hnums.contains(&-n) {
                max = n.abs();
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
        assert_eq!(Solution::find_max_k(vec![-1, 2, -3, 3]), 3);
        assert_eq!(Solution::find_max_k(vec![-1, 10, 6, 7, -7, 1]), 7);
        assert_eq!(Solution::find_max_k(vec![-10, 8, 6, 7, -2, -3]), -1);
    }
}
