//! <https://leetcode.com/problems/maximal-square/description/>

struct Solution {}

/// time-complexity : O(n*m)
/// space-complexity : O(n*m)
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let rows = matrix.len();
        let cols = matrix[0].len();

        let mut dp = vec![vec![0; cols + 1]; rows + 1];
        let mut max = 0;
        for i in 0..rows {
            for j in 0..cols {
                if matrix[i][j] == '1' {
                    dp[i + 1][j + 1] = dp[i][j].min(dp[i][j + 1]).min(dp[i + 1][j]) + 1;
                    max = max.max(dp[i + 1][j + 1]);
                }
            }
        }
        max * max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::maximal_square(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0']
            ]),
            4
        );
    }
}
