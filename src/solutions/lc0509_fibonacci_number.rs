//! <https://leetcode.com/problems/fibonacci-number/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    fn ifib(n: usize, dp: &mut [Option<i32>]) -> i32 {
        if n <= 1 {
            n as i32
        } else if let Some(n) = dp[n] {
            n
        } else {
            let total = Self::ifib(n - 1, dp) + Self::ifib(n - 2, dp);
            dp[n] = Some(total);
            total
        }
    }

    pub fn fib(n: i32) -> i32 {
        let mut dp = vec![None; (n + 1) as usize];
        Self::ifib(n as usize, &mut dp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::fib(2), 1);
        assert_eq!(Solution::fib(3), 2);
        assert_eq!(Solution::fib(4), 3);
    }
}
