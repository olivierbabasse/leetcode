//! <https://leetcode.com/problems/largest-combination-with-bitwise-and-greater-than-zero/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        (0..24)
            .map(|n| candidates.iter().filter(|&c| c & (1 << n) != 0).count())
            .max()
            .unwrap_or_default() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::largest_combination(vec![16, 17, 71, 62, 12, 24, 14]),
            4
        );
        assert_eq!(Solution::largest_combination(vec![8, 8]), 2);
    }
}
