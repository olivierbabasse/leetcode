//! <https://leetcode.com/problems/largest-submatrix-with-rearrangements/>

struct Solution {}

/// time-complexity : O(m*n)
/// space-complexity : O(m*n*log(n))
impl Solution {
    pub fn largest_submatrix(mut matrix: Vec<Vec<i32>>) -> i32 {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut max = 0;
        for i in 1..rows {
            for j in 0..cols {
                if matrix[i][j] != 0 {
                    matrix[i][j] = matrix[i - 1][j] + 1;
                }
            }
        }
        for mut row in matrix.into_iter() {
            row.sort_unstable_by(|a, b| b.cmp(a));
            for (j, c) in row.into_iter().enumerate() {
                max = max.max(c * (j as i32 + 1))
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::largest_submatrix(vec![vec![0, 0, 1], vec![1, 1, 1], vec![1, 0, 1]]),
            4
        );
        assert_eq!(Solution::largest_submatrix(vec![vec![1, 0, 1, 0, 1]]), 3);
        assert_eq!(
            Solution::largest_submatrix(vec![vec![1, 1, 0], vec![1, 0, 1]]),
            2
        );
    }
}
