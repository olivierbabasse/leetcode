//! <https://leetcode.com/problems/partitioning-into-minimum-number-of-deci-binary-numbers/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        n.chars().max().unwrap() as i32 - '0' as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::min_partitions("32".into()), 3);
        assert_eq!(Solution::min_partitions("82734".into()), 8);
        assert_eq!(Solution::min_partitions("27346209830709182346".into()), 9);
    }
}
