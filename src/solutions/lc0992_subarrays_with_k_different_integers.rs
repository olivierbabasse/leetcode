//! <https://leetcode.com/problems/subarrays-with-k-different-integers/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    fn subarrays_with_atmost_k(nums: &[i32], k: i32) -> i32 {
        let mut freqs: HashMap<i32, usize> = HashMap::new();

        let (mut count, mut begin) = (0, 0);
        for end in 0..nums.len() {
            *freqs.entry(nums[end]).or_default() += 1;

            while freqs.len() > k as usize {
                *freqs.entry(nums[begin]).or_default() -= 1;
                if freqs[&nums[begin]] == 0 {
                    freqs.remove_entry(&nums[begin]);
                }
                begin += 1;
            }

            count += end - begin + 1;
        }
        count as i32
    }

    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        Self::subarrays_with_atmost_k(&nums, k) - Self::subarrays_with_atmost_k(&nums, k - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::subarrays_with_k_distinct(vec![1, 2, 1, 2, 3], 2),
            7
        );
        assert_eq!(
            Solution::subarrays_with_k_distinct(vec![1, 2, 1, 3, 4], 3),
            3
        );
    }
}
