//! <https://leetcode.com/problems/kth-largest-element-in-an-array/>

use std::collections::BinaryHeap;

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut stack = BinaryHeap::new();
        nums.into_iter().for_each(|n| stack.push(n));
        for _ in 1..k {
            stack.pop();
        }
        *stack.peek().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
        assert_eq!(
            Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4),
            4
        );
    }
}
