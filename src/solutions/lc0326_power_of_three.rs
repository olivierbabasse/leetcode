//! <https://leetcode.com/problems/power-of-three/>

struct Solution {}

/// time-complexity : O(1)
/// space-complexity : O(1)
impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        let l = (n as f64).log(3.0);
        (l - l.round()).abs() < f64::EPSILON * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert!(Solution::is_power_of_three(9));
        assert!(Solution::is_power_of_three(27));
        assert!(Solution::is_power_of_three(243));
        assert!(!Solution::is_power_of_three(0));
        assert!(!Solution::is_power_of_three(-1));
        assert!(Solution::is_power_of_three(129140163));
    }
}
