//! <https://leetcode.com/problems/count-number-of-bad-pairs/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let mut counts: HashMap<i32, i64> = HashMap::new();
        let mut good = 0;
        let n = nums.len() as i64;

        nums.into_iter().enumerate().for_each(|(index, num)| {
            let e = counts.entry(num - index as i32).or_default();
            good += *e;
            *e += 1;
        });

        n * (n - 1) / 2 - good
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::count_bad_pairs(vec![4, 1, 3, 3]), 5);
        assert_eq!(Solution::count_bad_pairs(vec![1, 2, 3, 4, 5]), 0);
    }
}
