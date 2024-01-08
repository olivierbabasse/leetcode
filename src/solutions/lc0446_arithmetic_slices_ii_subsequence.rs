//! <https://leetcode.com/problems/arithmetic-slices-ii-subsequence/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O()
/// space-complexity : O()
impl Solution {
    fn rec(
        nums: &[i32],
        index: usize,
        prev: i32,
        diff: i64,
        seqlen: i32,
        cache: &mut HashMap<(usize, i32, i64, i32), i32>,
    ) -> i32 {
        let len = nums.len();
        if index >= len {
            0
        } else if let Some(&res) = cache.get(&(index, prev, diff, seqlen)) {
            res
        } else {
            let mut count = 0;
            for i in index..len {
                if seqlen == 0 {
                    count += Self::rec(nums, i + 1, nums[i], 0, 1, cache);
                } else if seqlen == 1 {
                    count +=
                        Self::rec(nums, i + 1, nums[i], nums[i] as i64 - prev as i64, 2, cache);
                } else if nums[i] as i64 - prev as i64 == diff {
                    count += 1 + Self::rec(nums, i + 1, nums[i], diff, seqlen + 1, cache);
                }
            }
            cache.insert((index, prev, diff, seqlen), count);
            count
        }
    }

    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        dbg!(&nums);
        Self::rec(&nums, 0, 0, 0, 0, &mut HashMap::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![2, 4, 6, 8, 10]),
            7
        );
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![7, 7, 7, 7, 7]),
            16
        );
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![0, 2000000000, -294967296]),
            0
        );
    }
}
