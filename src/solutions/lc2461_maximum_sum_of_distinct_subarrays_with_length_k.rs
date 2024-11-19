//! <https://leetcode.com/problems/maximum-sum-of-distinct-subarrays-with-length-k/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let (mut begin, mut end, mut res, mut current_sum) = (0, 0, 0i64, 0i64);
        let mut last_num_indexes = HashMap::<i64, i32>::new();
        while end < nums.len() as i32 {
            let num = nums[end as usize] as i64;
            let last_num_index = last_num_indexes.get(&num).unwrap_or(&-1);
            while begin <= *last_num_index || end - begin + 1 > k {
                current_sum -= nums[begin as usize] as i64;
                begin += 1;
            }
            last_num_indexes.insert(num, end);
            current_sum += nums[end as usize] as i64;
            if end - begin + 1 == k {
                res = res.max(current_sum);
            }
            end += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::maximum_subarray_sum(vec![1, 5, 4, 2, 9, 9, 9], 3),
            15
        );
        assert_eq!(Solution::maximum_subarray_sum(vec![4, 4, 4], 3), 0);
    }
}
