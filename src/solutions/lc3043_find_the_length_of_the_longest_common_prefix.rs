//! <https://leetcode.com/problems/find-the-length-of-the-longest-common-prefix/>

use std::collections::HashSet;

struct Solution {}

/// time-complexity : O(len(arr)*len(arr[x]))
/// space-complexity : O(len(arr)*len(arr[x]))
impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut arr1_prefixes = HashSet::new();

        arr1.into_iter().for_each(|mut a| {
            while a > 0 && !arr1_prefixes.contains(&a) {
                arr1_prefixes.insert(a);
                a /= 10;
            }
        });

        let mut longest = 0;

        arr2.into_iter().for_each(|mut a| {
            while a > 0 && !arr1_prefixes.contains(&a) {
                a /= 10;
            }
            if a > 0 {
                longest = longest.max(1 + (a as f64).log10() as i32);
            }
        });

        longest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::longest_common_prefix(vec![1, 10, 100], vec![1000]),
            3
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![1, 2, 3], vec![4, 4, 4]),
            0
        );
    }
}
