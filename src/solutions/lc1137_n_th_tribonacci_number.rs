//! <https://leetcode.com/problems/n-th-tribonacci-number/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    fn itrib(n: usize, dp: &mut [Option<i32>]) -> i32 {
        if n <= 1 {
            n as i32
        } else if n == 2 {
            1
        } else if let Some(n) = dp[n] {
            n
        } else {
            let total = Self::itrib(n - 1, dp) + Self::itrib(n - 2, dp) + Self::itrib(n - 3, dp);
            dp[n] = Some(total);
            total
        }
    }

    pub fn tribonacci(n: i32) -> i32 {
        let mut dp = vec![None; (n + 1) as usize];
        Self::itrib(n as usize, &mut dp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::tribonacci(4), 4);
        assert_eq!(Solution::tribonacci(25), 1389537);
    }
}
