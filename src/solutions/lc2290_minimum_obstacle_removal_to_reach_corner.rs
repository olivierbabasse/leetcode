//! <https://leetcode.com/problems/minimum-obstacle-removal-to-reach-corner/>

use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution {}

/// time-complexity : O(rows*cols*log(rows*cols))
/// space-complexity : O(rows*cols)
impl Solution {
    pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let (rows, cols) = (grid.len() as i32, grid[0].len() as i32);
        let mut costs = vec![vec![i32::MAX; cols as usize]; rows as usize];
        costs[0][0] = 0;

        let mut pq: BinaryHeap<(Reverse<i32>, i32, i32)> = BinaryHeap::new();
        pq.push((Reverse(0), 0, 0));
        while !pq.is_empty() {
            let (cost, row, col) = pq.pop().unwrap();

            for (drow, dcol) in directions {
                let (newrow, newcol) = (row + drow, col + dcol);
                if newrow >= 0 && newrow < rows && newcol >= 0 && newcol < cols {
                    let newcost = cost.0 + grid[newrow as usize][newcol as usize];
                    if newcost < costs[newrow as usize][newcol as usize] {
                        costs[newrow as usize][newcol as usize] = newcost;
                        pq.push((Reverse(newcost), newrow, newcol));
                    }
                }
            }
        }

        costs[rows as usize - 1][cols as usize - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::minimum_obstacles(vec![vec![0, 1, 1], vec![1, 1, 0], vec![1, 1, 0]]),
            2
        );
        assert_eq!(
            Solution::minimum_obstacles(vec![
                vec![0, 1, 0, 0, 0],
                vec![0, 1, 0, 1, 0],
                vec![0, 0, 0, 1, 0]
            ]),
            0
        );
    }
}
