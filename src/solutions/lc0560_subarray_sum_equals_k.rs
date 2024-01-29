//! <https://leetcode.com/problems/subarray-sum-equals-k/>

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = HashMap::new();
        map.insert(0, 1usize);
        let mut count = 0;
        let mut sum = 0;
        for n in nums {
            sum += n;
            count += map.get(&(sum - k)).unwrap_or(&0);
            *map.entry(sum).or_default() += 1;
        }

        count as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
        assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 3), 2);
    }
}
