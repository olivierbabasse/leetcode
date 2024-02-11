//! <https://leetcode.com/problems/cherry-pickup-ii/>

struct Solution {}

/// time-complexity : O(rows*cols^2)
/// space-complexity : O(rows*cols^2)
impl Solution {
    fn rec(
        grid: &Vec<Vec<i32>>,
        row: usize,
        col1: i32,
        col2: i32,
        cache: &mut Vec<Vec<Vec<Option<i32>>>>,
    ) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len() as i32;

        if row >= rows {
            return 0;
        }

        if let Some(res) = cache[row][col1 as usize][col2 as usize] {
            return res;
        }

        let mut res = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                if col1 + i >= 0 && col1 + i < cols && col2 + j >= 0 && col2 + j < cols {
                    res = res.max(Self::rec(grid, row + 1, col1 + i, col2 + j, cache));
                }
            }
        }

        res += grid[row][col1 as usize];
        if col1 != col2 {
            res += grid[row][col2 as usize];
        }

        cache[row][col1 as usize][col2 as usize] = Some(res);

        res
    }

    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut cache = vec![vec![vec![None; cols]; cols]; rows];
        Self::rec(&grid, 0, 0, cols as i32 - 1, &mut cache)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::cherry_pickup(vec![
                vec![3, 1, 1],
                vec![2, 5, 1],
                vec![1, 5, 5],
                vec![2, 1, 1]
            ]),
            24
        );
    }
}
