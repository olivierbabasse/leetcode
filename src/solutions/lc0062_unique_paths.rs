//! <https://leetcode.com/problems/unique-paths/>

struct Solution {}

/// time-complexity : O(n*m)
/// space-complexity : O(n*m)
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut dp = vec![1; (n * m) as usize];
        for j in 1..m {
            for i in 1..n {
                dp[(j * n + i) as usize] =
                    dp[((j - 1) * n + i) as usize] + dp[(j * n + i - 1) as usize];
            }
        }
        dp[((m - 1) * n + n - 1) as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::unique_paths(3, 7), 28);
    }
}
