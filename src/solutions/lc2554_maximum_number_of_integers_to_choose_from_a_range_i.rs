//! <https://leetcode.com/problems/maximum-number-of-integers-to-choose-from-a-range-i/>

use std::collections::HashSet;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn max_count(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
        let banned = banned.into_iter().collect::<HashSet<_>>();

        let (mut count, mut sum) = (0, 0);

        for num in 1..=n {
            if !banned.contains(&num) {
                if sum + num > max_sum {
                    return count;
                }
                sum += num;
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::max_count(vec![1, 6, 5], 5, 6), 2);
        assert_eq!(Solution::max_count(vec![1, 2, 3, 4, 5, 6, 7], 8, 1), 0);
        assert_eq!(Solution::max_count(vec![11], 7, 50), 7);
    }
}
