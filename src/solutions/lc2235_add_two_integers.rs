//! <https://leetcode.com/problems/add-two-integers/>

struct Solution {}

impl Solution {
    /// time-complexity : O(1)
    /// space-complexity : O(1)
    pub fn sum(num1: i32, num2: i32) -> i32 {
        num1 + num2
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::sum(12, 5), 17);
        assert_eq!(Solution::sum(-10, 4), -6);
    }
}
