//! <https://leetcode.com/problems/flip-columns-for-maximum-number-of-equal-rows/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(m*n)
/// space-complexity : O(m*n)
impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        let mut freqs: HashMap<Vec<u8>, i32> = HashMap::new();
        for row in matrix {
            let first = row[0];
            let pattern: Vec<u8> = row
                .into_iter()
                .map(|c| if c == first { 0u8 } else { 1u8 })
                .collect();
            *freqs.entry(pattern).or_default() += 1;
        }
        *freqs.values().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::max_equal_rows_after_flips(vec![vec![0, 1], vec![1, 1]]),
            1
        );
        assert_eq!(
            Solution::max_equal_rows_after_flips(vec![vec![0, 1], vec![1, 0]]),
            2
        );
        assert_eq!(
            Solution::max_equal_rows_after_flips(vec![vec![0, 0, 0], vec![0, 0, 1], vec![1, 1, 0]]),
            2
        );
    }
}
