//! <https://leetcode.com/problems/power-of-four/>

struct Solution {}

/// time-complexity : O(1)
/// space-complexity : O(1)
impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        if n <= 0 {
            return false;
        }

        if n & (n - 1) != 0 {
            return false;
        }

        (n & 0x55555555) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert!(Solution::is_power_of_four(16));
        assert!(!Solution::is_power_of_four(5));
        assert!(Solution::is_power_of_four(1));
    }
}
