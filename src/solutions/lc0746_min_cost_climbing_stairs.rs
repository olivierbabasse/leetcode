//! <https://leetcode.com/problems/min-cost-climbing-stairs/>

use std::cmp::min;
use std::collections::HashMap;

struct Solution {}

impl Solution {
    /// time-complexity : O(n)
    /// space-complexity : O(n)
    pub fn min_cost_climbing_stairs_rec(
        cost: &[i32],
        index: usize,
        costs: &mut HashMap<usize, i32>,
    ) -> i32 {
        if let Some(cost) = costs.get(&index) {
            return *cost;
        }

        let len = cost.len();
        if index >= len {
            0
        } else if let Some(cost) = costs.get(&index) {
            *cost
        } else {
            let cost = min(
                cost[index] + Self::min_cost_climbing_stairs_rec(cost, index + 1, costs),
                cost[index] + Self::min_cost_climbing_stairs_rec(cost, index + 2, costs),
            );
            costs.insert(index, cost);
            cost
        }
    }

    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut costs = HashMap::new();
        Self::min_cost_climbing_stairs_rec(&cost, 0, &mut costs);
        min(*costs.get(&0).unwrap(), *costs.get(&1).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
        assert_eq!(
            Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
    }
}
