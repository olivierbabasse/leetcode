//! <https://leetcode.com/problems/minimum-path-sum/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(n*m)
/// space-complexity : O(n*m)
impl Solution {
    fn rec(
        x: usize,
        y: usize,
        grid: &Vec<Vec<i32>>,
        cache: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        if let Some(&val) = cache.get(&(x, y)) {
            val
        } else if x >= grid[0].len() || y >= grid.len() {
            i32::MAX / 2
        } else if x == grid[0].len() - 1 && y == grid.len() - 1 {
            grid[y][x]
        } else {
            let val = grid[y][x]
                + i32::min(
                    Self::rec(x + 1, y, grid, cache),
                    Self::rec(x, y + 1, grid, cache),
                );
            cache.insert((x, y), val);
            val
        }
    }

    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut cache = HashMap::new();
        Self::rec(0, 0, &grid, &mut cache)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
            7
        );
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            12
        );
    }
}
