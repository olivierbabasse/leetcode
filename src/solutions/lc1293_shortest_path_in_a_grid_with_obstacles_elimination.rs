//! <https://leetcode.com/problems/shortest-path-in-a-grid-with-obstacles-elimination/>

use std::collections::VecDeque;

struct Solution {}

/// time-complexity : O(rows*cols*k)
/// space-complexity : O(rows*cols*k)
impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let (rows, cols) = (grid.len() as i32, grid[0].len() as i32);
        let mut visited = vec![vec![vec![false; k as usize + 1]; cols as usize]; rows as usize];
        let mut queue = VecDeque::new();
        queue.push_back((0, 0, k, 0));
        visited[0][0][k as usize] = true;
        while let Some((row, col, remaining, count)) = queue.pop_front() {
            if row == rows - 1 && col == cols - 1 {
                return count;
            }
            for (drow, dcol) in directions {
                let (newrow, newcol) = (row + drow, col + dcol);
                if newrow >= 0 && newrow < rows && newcol >= 0 && newcol < cols {
                    if grid[newrow as usize][newcol as usize] == 0
                        && !visited[newrow as usize][newcol as usize][remaining as usize]
                    {
                        visited[newrow as usize][newcol as usize][remaining as usize] = true;
                        queue.push_back((newrow, newcol, remaining, count + 1));
                    } else if remaining > 0
                        && !visited[newrow as usize][newcol as usize][remaining as usize - 1]
                    {
                        visited[newrow as usize][newcol as usize][remaining as usize - 1] = true;
                        queue.push_back((newrow, newcol, remaining - 1, count + 1));
                    }
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
            Solution::shortest_path(
                vec![
                    vec![0, 0, 0],
                    vec![1, 1, 0],
                    vec![0, 0, 0],
                    vec![0, 1, 1],
                    vec![0, 0, 0]
                ],
                1
            ),
            6
        );
        assert_eq!(
            Solution::shortest_path(vec![vec![0, 1, 1], vec![1, 1, 1], vec![1, 0, 0]], 1),
            -1
        );
    }
}
