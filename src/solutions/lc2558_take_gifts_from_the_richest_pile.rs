//! <https://leetcode.com/problems/take-gifts-from-the-richest-pile/>

use std::collections::BinaryHeap;

struct Solution {}

/// time-complexity : O((n+k)*log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut gifts: BinaryHeap<_> = gifts.into_iter().map(|g| g as i64).collect();
        for _ in 0..k as usize {
            let n = gifts.pop().unwrap();
            gifts.push((n as f64).sqrt() as i64);
        }
        gifts.into_iter().sum::<i64>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::pick_gifts(vec![25, 64, 9, 4, 100], 4), 29);
        assert_eq!(Solution::pick_gifts(vec![1, 1, 1, 1], 4), 4);
    }
}
