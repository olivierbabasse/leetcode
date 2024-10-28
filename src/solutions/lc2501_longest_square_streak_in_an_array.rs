//! <https://leetcode.com/problems/longest-square-streak-in-an-array/>

use std::collections::HashSet;

struct Solution {}

/// time-complexity : O(n^2)
/// space-complexity : O(n)
impl Solution {
    pub fn longest_square_streak(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let set: HashSet<_> = nums.iter().collect();
        for n in &nums {
            let mut cur = 0;
            let mut p = *n;
            while set.contains(&p) {
                cur += 1;
                if p > 10000 {
                    break;
                }
                p = p * p;
            }
            max = max.max(cur);
        }
        if max < 2 {
            -1
        } else {
            max
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::longest_square_streak(vec![4, 3, 6, 16, 8, 2]), 3);
        assert_eq!(Solution::longest_square_streak(vec![2, 3, 5, 6, 7]), -1);
    }
}
