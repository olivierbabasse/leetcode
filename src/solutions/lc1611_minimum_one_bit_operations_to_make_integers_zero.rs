//! <https://leetcode.com/problems/minimum-one-bit-operations-to-make-integers-zero/>

struct Solution {}

/// time-complexity : O(1)
/// space-complexity : O(1)
impl Solution {
    pub fn minimum_one_bit_operations(n: i32) -> i32 {
        let mut total = 0;
        let mut sign = 1;

        for i in (0..=31).rev() {
            if n & (1 << i) != 0 {
                total += sign * ((1 << (i + 1)) - 1);
                sign *= -1;
            }
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::minimum_one_bit_operations(3), 2);
        assert_eq!(Solution::minimum_one_bit_operations(6), 4);
    }
}
