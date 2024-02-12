//! <https://leetcode.com/problems/most-frequent-even-element/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        let mut freqs = HashMap::new();
        nums.into_iter()
            .filter(|n| n % 2 == 0)
            .for_each(|n| *freqs.entry(n).or_default() += 1);
        let mut min = i32::MAX;
        let mut minfreq = 0;
        freqs.into_iter().for_each(|(n, f)| {
            if f > minfreq || (n < min && f == minfreq) {
                min = n;
                minfreq = f;
            }
        });
        if min == i32::MAX {
            -1
        } else {
            min
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::most_frequent_even(vec![0, 1, 2, 2, 4, 4, 1]), 2);
        assert_eq!(Solution::most_frequent_even(vec![4, 4, 4, 9, 2, 4]), 4);
        assert_eq!(
            Solution::most_frequent_even(vec![29, 47, 21, 41, 13, 37, 25, 7]),
            -1
        );
    }
}
