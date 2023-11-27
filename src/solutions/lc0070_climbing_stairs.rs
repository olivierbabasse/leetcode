//! <https://leetcode.com/problems/climbing-stairs/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    fn stairs(n: usize, dp: &mut [Option<i32>]) -> i32 {
        if n <= 1 {
            1
        } else if let Some(n) = dp[n] {
            n
        } else {
            let total = Self::stairs(n - 1, dp) + Self::stairs(n - 2, dp);
            dp[n] = Some(total);
            total
        }
    }

    pub fn climb_stairs(n: i32) -> i32 {
        let mut dp = vec![None; (n + 1) as usize];
        Self::stairs(n as usize, &mut dp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
    }
}
