//! <https://leetcode.com/problems/unique-paths-iii/>

use std::collections::HashSet;

struct Solution {}

/// time-complexity : O()
/// space-complexity : O()
impl Solution {
    fn rec(
        grid: &Vec<Vec<i32>>,
        i: i32,
        j: i32,
        visited: &mut HashSet<(i32, i32)>,
        obstacles: usize,
    ) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();

        if i < 0 || i >= rows as i32 || j < 0 || j >= cols as i32 || visited.contains(&(i, j)) {
            return 0;
        }

        match grid[i as usize][j as usize] {
            2 => {
                if visited.len() == rows * cols - obstacles - 1 {
                    1
                } else {
                    0
                }
            }
            0 | 1 => {
                visited.insert((i, j));
                let res = Self::rec(grid, i + 1, j, visited, obstacles)
                    + Self::rec(grid, i - 1, j, visited, obstacles)
                    + Self::rec(grid, i, j + 1, visited, obstacles)
                    + Self::rec(grid, i, j - 1, visited, obstacles);
                visited.remove(&(i, j));
                res
            }
            _ => 0,
        }
    }

    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        let mut obstacles = 0;
        let (mut start_i, mut start_j) = (0, 0);
        for (i, row) in grid.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                if val == 1 {
                    start_i = i;
                    start_j = j;
                } else if val == -1 {
                    obstacles += 1;
                }
            }
        }
        Self::rec(
            &grid,
            start_i as i32,
            start_j as i32,
            &mut HashSet::new(),
            obstacles,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::unique_paths_iii(vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 2, -1]]),
            2
        );
        assert_eq!(
            Solution::unique_paths_iii(vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 2]]),
            4
        );
    }
}
