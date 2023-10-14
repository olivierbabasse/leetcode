//! <https://leetcode.com/problems/painting-the-walls/>

use std::cmp::{max, min};

struct Solution1 {}

impl Solution1 {
    /// time-complexity : O(n^2)
    /// space-complexity : O(n^2)
    fn calc_cost(
        cost: &Vec<i32>,
        time: &Vec<i32>,
        wall_index: i32,
        walls_remaining: i32,
        costs: &mut Vec<Option<i32>>,
    ) -> i32 {
        let len = (cost.len() + 1) as i32;
        if walls_remaining <= 0 {
            return 0;
        }

        if wall_index < 0 {
            return i32::MAX / 2;
        }

        if let Some(cost) = costs[(wall_index * len + walls_remaining) as usize] {
            return cost;
        }

        let cost_if_painting = cost[wall_index as usize]
            + Self::calc_cost(
                cost,
                time,
                wall_index - 1,
                walls_remaining - time[wall_index as usize] - 1,
                costs,
            );
        let cost_if_not_painting =
            Self::calc_cost(cost, time, wall_index - 1, walls_remaining, costs);
        let best_cost = min(cost_if_painting, cost_if_not_painting);

        costs[(wall_index * len + walls_remaining) as usize] = Some(best_cost);
        best_cost
    }

    pub fn paint_walls(cost: Vec<i32>, time: Vec<i32>) -> i32 {
        let len = cost.len();
        let mut costs = vec![None; (len + 1) * len];

        Self::calc_cost(&cost, &time, (len - 1) as i32, len as i32, &mut costs)
    }
}

struct Solution2 {}

impl Solution2 {
    /// time-complexity : O(n^2)
    /// space-complexity : O(n)
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
    use super::{Solution1, Solution2};

    #[test]
    fn test_cases_1() {
        assert_eq!(
            Solution1::paint_walls(vec![1, 2, 3, 2], vec![1, 2, 3, 2]),
            3
        );
        assert_eq!(
            Solution1::paint_walls(vec![2, 3, 4, 2], vec![1, 1, 1, 1]),
            4
        );
    }

    #[test]
    fn test_cases_2() {
        assert_eq!(
            Solution2::paint_walls(vec![1, 2, 3, 2], vec![1, 2, 3, 2]),
            3
        );
        assert_eq!(
            Solution2::paint_walls(vec![2, 3, 4, 2], vec![1, 1, 1, 1]),
            4
        );
    }
}
