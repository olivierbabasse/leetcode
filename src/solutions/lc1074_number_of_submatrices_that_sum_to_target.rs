//! <https://leetcode.com/problems/number-of-submatrices-that-sum-to-target/>

struct Solution {}

impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let rows = matrix.len();
        let cols = matrix[0].len();

        let mut prefix_sum = vec![vec![0; cols + 1]; rows + 1];
        for i in 1..=rows {
            for j in 1..=cols {
                prefix_sum[i][j] =
                    prefix_sum[i - 1][j] + prefix_sum[i][j - 1] + matrix[i - 1][j - 1]
                        - prefix_sum[i - 1][j - 1];
            }
        }

        let mut count = 0;
        for row1 in 0..rows {
            for row2 in row1..rows {
                for col1 in 0..cols {
                    for col2 in col1..cols {
                        if prefix_sum[row2 + 1][col2 + 1]
                            - prefix_sum[row2 + 1][col1]
                            - prefix_sum[row1][col2 + 1]
                            + prefix_sum[row1][col1]
                            == target
                        {
                            count += 1;
                        }
                    }
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::num_submatrix_sum_target(
                vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 0]],
                0
            ),
            4
        );
        assert_eq!(
            Solution::num_submatrix_sum_target(vec![vec![1, -1], vec![-1, 1]], 0),
            5
        );
        assert_eq!(Solution::num_submatrix_sum_target(vec![vec![904]], 0), 0);
    }
}
