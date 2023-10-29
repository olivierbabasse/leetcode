//! <https://leetcode.com/problems/poor-pigs/>

struct Solution {}

/// time-complexity : O(1)
/// space-complexity : O(1)
impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        ((buckets as f64).log2() / ((1 + minutes_to_test / minutes_to_die) as f64).log2()).ceil()
            as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::poor_pigs(4, 15, 15), 2);
        assert_eq!(Solution::poor_pigs(4, 15, 30), 2);
    }
}
