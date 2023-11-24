//! <https://leetcode.com/problems/minimum-cost-homecoming-of-a-robot-in-a-grid/>

use std::cmp::Ordering;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn min_cost(
        start_pos: Vec<i32>,
        home_pos: Vec<i32>,
        row_costs: Vec<i32>,
        col_costs: Vec<i32>,
    ) -> i32 {
        let (mut posy, mut posx) = (start_pos[0], start_pos[1]);
        let (desty, destx) = (home_pos[0], home_pos[1]);
        let mut cost = 0;

        while posx != destx || posy != desty {
            match posx.cmp(&destx) {
                Ordering::Less => {
                    posx += 1;
                    cost += col_costs[posx as usize];
                }
                Ordering::Greater => {
                    posx -= 1;
                    cost += col_costs[posx as usize];
                }
                _ => {}
            }
            match posy.cmp(&desty) {
                Ordering::Less => {
                    posy += 1;
                    cost += row_costs[posy as usize];
                }
                Ordering::Greater => {
                    posy -= 1;
                    cost += row_costs[posy as usize];
                }
                _ => {}
            }
        }

        cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::min_cost(vec![1, 0], vec![2, 3], vec![5, 4, 3], vec![8, 2, 6, 7]),
            18
        );
    }
}
