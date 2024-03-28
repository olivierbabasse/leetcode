//! <https://leetcode.com/problems/length-of-longest-subarray-with-at-most-k-frequency/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut max = 0;
        let (mut begin, mut end) = (0, 0);
        let mut freqs: HashMap<i32, i32> = HashMap::new();

        while end < nums.len() {
            *freqs.entry(nums[end]).or_default() += 1;

            while freqs.get(&nums[end]).unwrap_or(&0) > &k {
                *freqs.entry(nums[begin]).or_default() -= 1;
                begin += 1;
            }

            max = max.max(end - begin + 1);

            end += 1;
        }

        max as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::max_subarray_length(vec![3, 1, 1], 1), 2);
        assert_eq!(
            Solution::max_subarray_length(vec![1, 2, 3, 1, 2, 3, 1, 2], 2),
            6
        );
        assert_eq!(
            Solution::max_subarray_length(vec![1, 2, 1, 2, 1, 2, 1, 2], 1),
            2
        );
        assert_eq!(
            Solution::max_subarray_length(vec![5, 5, 5, 5, 5, 5, 5], 4),
            4
        );
    }
}
