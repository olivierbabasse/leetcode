//! <https://leetcode.com/problems/cherry-pickup/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O()
/// space-complexity : O()
impl Solution {
    fn rec(
        grid: &Vec<Vec<i32>>,
        row1: usize,
        col1: usize,
        row2: usize,
        col2: usize,
        cache: &mut HashMap<(usize, usize, usize, usize), i32>,
    ) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();

        if let Some(&res) = cache.get(&(row1, col1, row2, col2)) {
            return res;
        }

        if row1 == rows - 1 && col1 == cols - 1 && row2 == rows - 1 && col2 == cols - 1 {
            return grid[row1][col1];
        }

        if row1 >= rows
            || row2 >= rows
            || col1 >= cols
            || col2 >= cols
            || grid[row1][col1] == -1
            || grid[row2][col2] == -1
        {
            return i32::MIN / 2;
        }

        const INCRS: [[usize; 4]; 4] = [[1, 0, 1, 0], [0, 1, 0, 1], [1, 0, 0, 1], [0, 1, 1, 0]];

        let mut res = i32::MIN / 2;
        for inc in &INCRS {
            let r1 = row1 + inc[0];
            let c1 = col1 + inc[1];
            let r2 = row2 + inc[2];
            let c2 = col2 + inc[3];

            res = res.max(Self::rec(grid, r1, c1, r2, c2, cache));
        }

        res += grid[row1][col1];
        if row1 != row2 || col1 != col2 {
            res += grid[row2][col2];
        }

        cache.insert((row1, col1, row2, col2), res);

        res
    }

    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let mut cache = HashMap::new();
        let res = Self::rec(&grid, 0, 0, 0, 0, &mut cache);
        if res > 0 {
            res
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::cherry_pickup(vec![vec![0, 1, -1], vec![1, 0, -1], vec![1, 1, 1]]),
            5
        );
        assert_eq!(
            Solution::cherry_pickup(vec![vec![1, 1, -1], vec![1, -1, 1], vec![-1, 1, 1]]),
            0
        );
    }
}
