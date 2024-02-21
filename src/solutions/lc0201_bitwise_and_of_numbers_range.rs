//! <https://leetcode.com/problems/bitwise-and-of-numbers-range/>

struct Solution {}

/// time-complexity : O(1)
/// space-complexity : O(1)
impl Solution {
    pub fn range_bitwise_and(mut left: i32, mut right: i32) -> i32 {
        let mut count = 0;
        while left != right {
            count += 1;
            left >>= 1;
            right >>= 1;
        }
        left << count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::range_bitwise_and(5, 7), 4);
        assert_eq!(Solution::range_bitwise_and(0, 0), 0);
        assert_eq!(Solution::range_bitwise_and(1, 2147483647), 0);
    }
}
