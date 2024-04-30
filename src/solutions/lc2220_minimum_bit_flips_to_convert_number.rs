//! <https://leetcode.com/problems/minimum-bit-flips-to-convert-number/description/>

struct Solution {}

/// time-complexity : O(1)
/// space-complexity : O(1)
impl Solution {
    pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
        (start ^ goal).count_ones() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::min_bit_flips(10, 7), 3);
        assert_eq!(Solution::min_bit_flips(3, 4), 3);
    }
}
