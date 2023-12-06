//! <https://leetcode.com/problems/calculate-money-in-leetcode-bank/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn total_money(n: i32) -> i32 {
        (0..n).fold(0, |total, day| total + 1 + day % 7 + day / 7)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::total_money(4), 10);
        assert_eq!(Solution::total_money(10), 37);
        assert_eq!(Solution::total_money(20), 96);
    }
}
