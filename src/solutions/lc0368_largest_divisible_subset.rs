//! <https://leetcode.com/problems/largest-divisible-subset/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O()
/// space-complexity : O()
impl Solution {
    fn rec(
        nums: &[i32],
        index: usize,
        largest: Option<i32>,
        cache: &mut HashMap<(usize, Option<i32>), Vec<i32>>,
    ) -> Vec<i32> {
        if index >= nums.len() {
            return Vec::new();
        }

        if let Some(res) = cache.get(&(index, largest)) {
            return res.clone();
        }

        let sol1 = Self::rec(nums, index + 1, largest, cache);
        let mut sol2 = Vec::new();

        if let Some(largest) = largest {
            if nums[index] % largest == 0 {
                sol2.push(nums[index]);
                sol2.append(&mut Self::rec(nums, index + 1, Some(nums[index]), cache));
            }
        } else {
            sol2.push(nums[index]);
            sol2.append(&mut Self::rec(nums, index + 1, Some(nums[index]), cache));
        }

        let s = if sol1.len() > sol2.len() { sol1 } else { sol2 };
        cache.insert((index, largest), s.clone());
        s
    }

    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        let mut cache = HashMap::new();
        Self::rec(&nums, 0, None, &mut cache)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::largest_divisible_subset(vec![1, 2, 3]),
            vec![1, 2]
        );
        assert_eq!(
            Solution::largest_divisible_subset(vec![1, 2, 4, 8]),
            vec![1, 2, 4, 8]
        );
        assert_eq!(
            Solution::largest_divisible_subset(vec![3, 4, 16, 8]),
            vec![4, 8, 16]
        );
        assert_eq!(
            Solution::largest_divisible_subset(vec![5, 9, 18, 54, 108, 540, 90, 180, 360, 720]),
            vec![9, 18, 90, 180, 360, 720]
        );
    }
}
