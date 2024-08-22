//! <https://leetcode.com/problems/number-complement/>

struct Solution {}

/// time-complexity : O(1)
/// space-complexity : O(1)
impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        num ^ ((1 << (32 - num.leading_zeros())) - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::find_complement(5), 2);
        assert_eq!(Solution::find_complement(1), 0);
    }
}
