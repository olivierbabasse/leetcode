//! <https://leetcode.com/problems/unique-paths-ii/>

struct Solution {}

/// time-complexity : O(n*m)
/// space-complexity : O(n*m)
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let n = obstacle_grid[0].len();
        let m = obstacle_grid.len();
        let mut dp = vec![0; n * m];
        dp[0] = 1;
        for j in 0..m {
            for i in 0..n {
                dp[j * n + i] = if obstacle_grid[j][i] == 1 {
                    0
                } else if i == 0 && j == 0 {
                    dp[0]
                } else if j == 0 {
                    dp[i - 1]
                } else if i == 0 {
                    dp[(j - 1) * n]
                } else {
                    dp[(j - 1) * n + i] + dp[j * n + i - 1]
                };
            }
        }
        dp[(m - 1) * n + n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0]
            ]),
            2
        );
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![0, 1], vec![0, 0]]),
            1
        );
        assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![1, 0]]), 0);
    }
}
