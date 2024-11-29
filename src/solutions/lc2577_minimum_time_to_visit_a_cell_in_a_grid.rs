//! <https://leetcode.com/problems/minimum-time-to-visit-a-cell-in-a-grid/>

use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution {}

/// time-complexity : O(rows*cols*log(rows*cols))
/// space-complexity : O(rows*cols)
impl Solution {
    pub fn minimum_time(grid: Vec<Vec<i32>>) -> i32 {
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let (rows, cols) = (grid.len() as i32, grid[0].len() as i32);
        let mut visited = vec![vec![false; cols as usize]; rows as usize];

        if grid[0][1] > 1 && grid[1][0] > 1 {
            return -1;
        }

        let mut pq = BinaryHeap::new();
        pq.push((Reverse(grid[0][0]), 0, 0));
        while let Some((time, row, col)) = pq.pop() {
            if row == rows - 1 && col == cols - 1 {
                return time.0;
            }

            if visited[row as usize][col as usize] {
                continue;
            }
            visited[row as usize][col as usize] = true;

            for (drow, dcol) in &directions {
                let (newrow, newcol) = (row + drow, col + dcol);
                if newrow >= 0 && newrow < rows && newcol >= 0 && newcol < cols {
                    let wait = if (grid[newrow as usize][newcol as usize] - time.0) % 2 == 0 {
                        1
                    } else {
                        0
                    };
                    let next = i32::max(grid[newrow as usize][newcol as usize] + wait, time.0 + 1);
                    pq.push((Reverse(next), newrow, newcol));
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::minimum_time(vec![vec![0, 1, 3, 2], vec![5, 1, 2, 5], vec![4, 3, 8, 6]]),
            7
        );
        assert_eq!(
            Solution::minimum_time(vec![vec![0, 2, 4], vec![3, 2, 1], vec![1, 0, 4]]),
            -1
        );
    }
}
