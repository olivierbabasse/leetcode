//! <https://leetcode.com/problems/count-nice-pairs-in-an-array/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    fn rev(mut num: i32) -> i32 {
        let mut res = 0;
        while num > 0 {
            res = res * 10 + num % 10;
            num /= 10;
        }
        res
    }

    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        let mut counts: HashMap<i32, i32> = HashMap::new();
        let mut total = 0;

        nums.into_iter().for_each(|num| {
            let e = counts.entry(num - Self::rev(num)).or_default();
            total = (total + *e) % 1000000007;
            *e += 1;
        });

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::count_nice_pairs(vec![42, 11, 1, 97]), 2);
        assert_eq!(Solution::count_nice_pairs(vec![13, 10, 35, 24, 76]), 4);
    }
}
