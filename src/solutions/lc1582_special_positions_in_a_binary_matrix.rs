//! <https://leetcode.com/problems/special-positions-in-a-binary-matrix/>

struct Solution {}

/// time-complexity : O(m*n)
/// space-complexity : O(m+n)
impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let mut total = 0;
        let rows = mat.len();
        let cols = mat[0].len();
        let row_sums = mat
            .iter()
            .map(|v| v.iter().sum::<i32>())
            .collect::<Vec<_>>();
        let col_sums = (0..cols)
            .map(|c| (0..rows).map(|r| mat[r][c]).sum::<i32>())
            .collect::<Vec<_>>();

        for (j, &row_sum) in row_sums.iter().enumerate() {
            for (i, &col_sum) in col_sums.iter().enumerate() {
                if mat[j][i] == 1 && row_sum == 1 && col_sum == 1 {
                    total += 1;
                }
            }
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::num_special(vec![vec![1, 0, 0], vec![0, 0, 1], vec![1, 0, 0]]),
            1
        );
        assert_eq!(
            Solution::num_special(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]),
            3
        );
    }
}
