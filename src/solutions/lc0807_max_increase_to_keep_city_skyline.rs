//! <https://leetcode.com/problems/max-increase-to-keep-city-skyline/>

struct Solution {}

/// time-complexity : O(n^2)
/// space-complexity : O(n)
impl Solution {
    #[allow(clippy::needless_range_loop)]
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut max_by_row = vec![0; rows];
        let mut max_by_col = vec![0; cols];

        for i in 0..rows {
            for j in 0..cols {
                max_by_row[i] = max_by_row[i].max(grid[i][j]);
                max_by_col[j] = max_by_col[j].max(grid[i][j]);
            }
        }

        let mut res = 0;
        for i in 0..rows {
            for j in 0..cols {
                let m = max_by_row[i].min(max_by_col[j]);
                if grid[i][j] < m {
                    res += m - grid[i][j];
                }
            }
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
            Solution::max_increase_keeping_skyline(vec![
                vec![3, 0, 8, 4],
                vec![2, 4, 5, 7],
                vec![9, 2, 6, 3],
                vec![0, 3, 1, 0]
            ]),
            35
        );
    }
}
