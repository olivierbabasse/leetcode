//! <https://leetcode.com/problems/maximal-score-after-applying-k-operations/>

use std::collections::BinaryHeap;

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        let mut nums = nums
            .into_iter()
            .map(|n| n as i64)
            .collect::<BinaryHeap<i64>>();
        let mut score = 0i64;
        for _ in 0..k as usize {
            let val = nums.pop().unwrap();
            score += val;
            nums.push((val as f64 / 3.0).ceil() as i64);
        }
        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::max_kelements(vec![10, 10, 10, 10, 10], 5), 50);
        assert_eq!(Solution::max_kelements(vec![1, 10, 3, 3, 3], 3), 17);
    }
}
