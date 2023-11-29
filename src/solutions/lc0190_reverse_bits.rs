//! <https://leetcode.com/problems/reverse-bits/>

struct Solution {}

/// time-complexity : O(1)
/// space-complexity : O(n)
impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut res = 0;
        for i in 0..32 {
            if x & (1 << (31 - i)) != 0 {
                res += 1 << i;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::reverse_bits(0b00000010100101000001111010011100),
            964176192
        );
        assert_eq!(
            Solution::reverse_bits(0b11111111111111111111111111111101),
            3221225471
        );
    }
}
