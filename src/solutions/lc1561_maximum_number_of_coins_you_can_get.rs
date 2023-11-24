//! <https://leetcode.com/problems/maximum-number-of-coins-you-can-get/>

use std::collections::VecDeque;

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn max_coins(mut piles: Vec<i32>) -> i32 {
        piles.sort_unstable();
        let mut piles = piles.into_iter().collect::<VecDeque<_>>();

        let mut total = 0;
        while piles.len() >= 3 {
            piles.pop_back();
            total += piles.pop_back().unwrap();
            piles.pop_front();
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::max_coins(vec![2, 4, 1, 2, 7, 8]), 9);
        assert_eq!(Solution::max_coins(vec![2, 4, 5]), 4);
        assert_eq!(Solution::max_coins(vec![9, 8, 7, 6, 5, 1, 2, 3, 4]), 18);
    }
}
