//! <https://leetcode.com/problems/make-sum-divisible-by-p/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O()
/// space-complexity : O()
impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let len = nums.len() as i32;
        let target = nums.iter().fold(0, |s, &n| (s + n) % p);
        if target == 0 {
            return 0;
        }

        let mut mods = HashMap::new();
        mods.insert(0, -1i32);

        let mut minlen = len;
        let mut cursum = 0;
        for (i, num) in nums.into_iter().enumerate() {
            cursum = (cursum + num) % p;
            let needed = (cursum - target + p) % p;
            if let Some(&val) = mods.get(&needed) {
                minlen = minlen.min(i as i32 - val);
            }
            mods.insert(cursum, i as i32);
        }

        if minlen == len {
            return -1;
        }
        minlen
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::min_subarray(vec![3, 1, 4, 2], 6), 1);
        assert_eq!(Solution::min_subarray(vec![6, 3, 5, 2], 9), 2);
        assert_eq!(Solution::min_subarray(vec![1, 2, 3], 3), 0);
    }
}
