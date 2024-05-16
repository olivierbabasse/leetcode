//! <https://leetcode.com/problems/find-the-safest-path-in-a-grid/>

use std::collections::{BinaryHeap, VecDeque};

struct Solution {}

/// time-complexity : O(log(n)*n^2)
/// space-complexity : O(n^2)
impl Solution {
    pub fn maximum_safeness_factor(mut grid: Vec<Vec<i32>>) -> i32 {
        const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

        let rows = grid.len();
        let cols = grid[0].len();

        let valid_cell =
            |row: i32, col: i32| row >= 0 && col >= 0 && row < rows as i32 && col < cols as i32;

        let mut queue = VecDeque::new();
        for (r, row) in grid.iter_mut().enumerate() {
            for (c, val) in row.iter_mut().enumerate() {
                if *val == 1 {
                    queue.push_back((r, c));
                    *val = 0;
                } else {
                    *val = -1;
                }
            }
        }

        while !queue.is_empty() {
            let len = queue.len();
            for _ in 0..len {
                if let Some((r, c)) = queue.pop_front() {
                    for d in DIRECTIONS {
                        let dr = r as i32 + d.0;
                        let dc = c as i32 + d.1;
                        let val = grid[r][c];
                        if valid_cell(dr, dc) && grid[dr as usize][dc as usize] == -1 {
                            let (dr, dc) = (dr as usize, dc as usize);
                            grid[dr][dc] = val + 1;
                            queue.push_back((dr, dc));
                        }
                    }
                }
            }
        }

        let mut pqueue = BinaryHeap::new();
        pqueue.push((grid[0][0], 0, 0));
        grid[0][0] = -1;

        while let Some((safety, r, c)) = pqueue.pop() {
            if r == rows - 1 && c == cols - 1 {
                return safety;
            }

            for d in DIRECTIONS {
                let dr = r as i32 + d.0;
                let dc = c as i32 + d.1;
                if valid_cell(dr, dc) && grid[dr as usize][dc as usize] != -1 {
                    let (dr, dc) = (dr as usize, dc as usize);
                    pqueue.push((safety.min(grid[dr][dc]), dr, dc));
                    grid[dr][dc] = -1;
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
            Solution::maximum_safeness_factor(vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 1]]),
            0
        );
        assert_eq!(
            Solution::maximum_safeness_factor(vec![vec![0, 0, 1], vec![0, 0, 0], vec![0, 0, 0]]),
            2
        );
        assert_eq!(
            Solution::maximum_safeness_factor(vec![
                vec![0, 0, 0, 1],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![1, 0, 0, 0]
            ]),
            2
        );
    }
}
