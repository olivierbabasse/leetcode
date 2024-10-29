//! <https://leetcode.com/problems/maximum-number-of-moves-in-a-grid/>

struct Solution {}

/// time-complexity : O(len(grid)*len(grid[x]))
/// space-complexity : O(len(grid)*len(grid[x]))
impl Solution {
    fn rec(grid: &[Vec<i32>], row: usize, col: usize, cache: &mut [Vec<Option<i32>>]) -> i32 {
        if let Some(res) = cache[row][col] {
            return res;
        }

        let mut res = 0;
        for i in -1..=1 {
            let (r, c) = (row as i32 + i, col + 1);
            if r >= 0
                && (r as usize) < grid.len()
                && c < grid[0].len()
                && grid[r as usize][c] > grid[row][col]
            {
                res = res.max(1 + Self::rec(grid, r as usize, c, cache));
            }
        }
        cache[row][col] = Some(res);
        res
    }

    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let mut cache = vec![vec![None; grid[0].len()]; grid.len()];
        let mut res = 0;
        for i in 0..grid.len() {
            res = res.max(Self::rec(&grid, i, 0, &mut cache));
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::max_moves(vec![
                vec![2, 4, 3, 5],
                vec![5, 4, 9, 3],
                vec![3, 4, 2, 11],
                vec![10, 9, 13, 15]
            ]),
            3
        );
    }
}
