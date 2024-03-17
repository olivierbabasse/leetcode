//! <https://leetcode.com/problems/contiguous-array/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut counts = HashMap::new();
        let mut count = 0i32;
        let mut max = 0i32;
        counts.insert(0, -1);
        for (i, n) in nums.into_iter().enumerate() {
            if n > 0 {
                count += 1
            } else {
                count -= 1
            };
            if let Some(&c) = counts.get(&count) {
                max = max.max(i as i32 - c);
            } else {
                counts.insert(count, i as i32);
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::find_max_length(vec![0, 1]), 2);
        assert_eq!(Solution::find_max_length(vec![0, 1, 0]), 2);
    }
}
