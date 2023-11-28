//! <https://leetcode.com/problems/delete-and-earn/>

use std::collections::BTreeMap;

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(n)
impl Solution {
    fn dp(index: usize, frequencies: &Vec<(i32, i32)>, cache: &mut Vec<Option<i32>>) -> i32 {
        if index >= frequencies.len() {
            0
        } else if let Some(val) = cache[index] {
            val
        } else {
            let val = if index < frequencies.len() - 1
                && frequencies[index + 1].0 == frequencies[index].0 + 1
            {
                i32::max(
                    Self::dp(index + 1, frequencies, cache),
                    frequencies[index].0 * frequencies[index].1
                        + Self::dp(index + 2, frequencies, cache),
                )
            } else {
                frequencies[index].0 * frequencies[index].1
                    + Self::dp(index + 1, frequencies, cache)
            };
            cache[index] = Some(val);
            val
        }
    }

    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mut value_and_count: BTreeMap<i32, i32> = BTreeMap::new();
        nums.into_iter()
            .for_each(|v| *value_and_count.entry(v).or_default() += 1);
        let frequencies = value_and_count.into_iter().collect::<Vec<_>>();

        let mut cache = vec![None; frequencies.len() + 1];
        Self::dp(0, &frequencies, &mut cache)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::delete_and_earn(vec![3, 4, 2]), 6);
        assert_eq!(Solution::delete_and_earn(vec![2, 2, 3, 3, 3, 4]), 9);
    }
}
