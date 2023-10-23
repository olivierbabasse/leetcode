//! <https://leetcode.com/problems/power-of-two/>

struct Solution {}

/// time-complexity : O(1)
/// space-complexity : O(1)
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        n & (n - 1) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert!(!Solution::is_power_of_two(0));
        assert!(Solution::is_power_of_two(1));
        assert!(Solution::is_power_of_two(16));
        assert!(!Solution::is_power_of_two(3));
    }
}
