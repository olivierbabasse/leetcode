//! <https://leetcode.com/problems/binary-subarrays-with-sum/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let mut sums = HashMap::new();
        let (mut count, mut sum) = (0, 0);
        sums.insert(0, 1);
        for n in nums {
            sum += n;
            count += sums.get(&(sum - goal)).unwrap_or(&0);
            *sums.entry(sum).or_default() += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::num_subarrays_with_sum(vec![1, 0, 1, 0, 1], 2), 4);
        assert_eq!(Solution::num_subarrays_with_sum(vec![0, 0, 0, 0, 0], 0), 15);
    }
}
