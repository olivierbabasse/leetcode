//! <https://leetcode.com/problems/count-of-matches-in-tournament/>

struct Solution {}

/// time-complexity : O(1)
/// space-complexity : O(1)
impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        n - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::number_of_matches(7), 6);
        assert_eq!(Solution::number_of_matches(14), 13);
    }
}
