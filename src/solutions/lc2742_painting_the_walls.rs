//! <https://leetcode.com/problems/painting-the-walls/>

use std::cmp::{max, min};

struct Solution {}

/// time-complexity : O(n^2)
/// space-complexity : O(n)
impl Solution {
    pub fn paint_walls(cost: Vec<i32>, time: Vec<i32>) -> i32 {
        let len = cost.len();

        let mut min_costs = Vec::new();
        min_costs.resize(len + 1, i32::MAX / 2);
        min_costs[0] = 0;

        for i in 0..len {
            for wall in (0..=len).rev() {
                min_costs[wall] = min(
                    min_costs[wall],
                    cost[i] + min_costs[max(0, wall as i32 - time[i] - 1) as usize],
                );
            }
        }

        min_costs[len]
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::paint_walls(vec![1, 2, 3, 2], vec![1, 2, 3, 2]), 3);
        assert_eq!(Solution::paint_walls(vec![2, 3, 4, 2], vec![1, 1, 1, 1]), 4);
    }
}
