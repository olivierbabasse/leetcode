//! <https://leetcode.com/problems/unique-number-of-occurrences/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut freqs: HashMap<i32, usize> = HashMap::new();
        arr.into_iter().for_each(|v| {
            *freqs.entry(v).or_default() += 1;
        });
        let mut freqs = freqs.into_values().collect::<Vec<_>>();
        freqs.sort_unstable();
        freqs.windows(2).all(|w| w[0] != w[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert!(Solution::unique_occurrences(vec![1, 2, 2, 1, 1, 3]));
        assert!(!Solution::unique_occurrences(vec![1, 2]));
        assert!(Solution::unique_occurrences(vec![
            -3, 0, 1, -3, 1, 1, 1, -3, 10, 0
        ]));
    }
}
