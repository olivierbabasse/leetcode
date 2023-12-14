//! <https://leetcode.com/problems/difference-between-ones-and-zeros-in-row-and-column/>

struct Solution {}

/// time-complexity : O(m*n)
/// space-complexity : O(m+n)
impl Solution {
    pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let rows = grid.len();
        let cols = grid[0].len();
        let row_sums = grid
            .iter()
            .map(|v| v.iter().sum::<i32>())
            .collect::<Vec<_>>();
        let col_sums = (0..cols)
            .map(|c| (0..rows).map(|r| grid[r][c]).sum::<i32>())
            .collect::<Vec<_>>();

        let mut res = vec![vec![0; cols]; rows];
        for (i, &row_sum) in row_sums.iter().enumerate() {
            for (j, &col_sum) in col_sums.iter().enumerate() {
                res[i][j] = row_sum + col_sum - (cols as i32 - row_sum) - (rows as i32 - col_sum);
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
            Solution::ones_minus_zeros(vec![vec![0, 1, 1], vec![1, 0, 1], vec![0, 0, 1]]),
            vec![vec![0, 0, 4], vec![0, 0, 4], vec![-2, -2, 2]]
        )
    }
}
