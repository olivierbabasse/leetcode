//! <https://leetcode.com/problems/score-after-flipping-matrix/>

struct Solution {}

/// time-complexity : O(m*n)
/// space-complexity : O(1)
impl Solution {
    pub fn matrix_score(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        for row in grid.iter_mut() {
            if row[0] == 0 {
                row.iter_mut().for_each(|n| *n = 1 - *n);
            }
        }

        for j in 1..n {
            if m as i32 - 2 * (0..m).map(|i| grid[i][j]).sum::<i32>() > 0 {
                (0..m).for_each(|i| grid[i][j] = 1 - grid[i][j]);
            }
        }

        grid.into_iter()
            .map(|row| {
                row.iter()
                    .rev()
                    .enumerate()
                    .fold(0, |acc, (pos, &val)| acc + (1 << pos) * val)
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::matrix_score(vec![vec![0, 0, 1, 1], vec![1, 0, 1, 0], vec![1, 1, 0, 0]]),
            39
        );
        assert_eq!(Solution::matrix_score(vec![vec![0]]), 1);
    }
}
