//! <https://leetcode.com/problems/number-of-1-bits/>

struct Solution {}

/// time-complexity : O(1)
/// space-complexity : O(1)
impl Solution {
    #[allow(non_snake_case)]
    pub fn hammingWeight(mut n: u32) -> i32 {
        n = n - ((n >> 1) & 0x55555555);
        n = (n & 0x33333333) + ((n >> 2) & 0x33333333);
        n = (n + (n >> 4)) & 0x0F0F0F0F;
        n = n + (n >> 8);
        n = n + (n >> 16);
        (n & 0x0000003F) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::hammingWeight(0b00000000000000000000000000001011),
            3
        );
        assert_eq!(
            Solution::hammingWeight(0b00000000000000000000000010000000),
            1
        );
        assert_eq!(
            Solution::hammingWeight(0b11111111111111111111111111111101),
            31
        );
    }
}
