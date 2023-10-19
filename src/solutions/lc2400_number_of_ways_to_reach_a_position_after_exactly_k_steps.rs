//! <https://leetcode.com/problems/number-of-ways-to-reach-a-position-after-exactly-k-steps/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(k * arraylen)
/// space-complexity : O(k * arraylen)
impl Solution {
    fn dp(cur_pos: i32, end_pos: i32, k: i32, memo: &mut HashMap<(i32, i32), i32>) -> i32 {
        if k == 0 {
            if cur_pos == end_pos {
                1
            } else {
                0
            }
        } else if let Some(res) = memo.get(&(cur_pos, k)) {
            *res
        } else {
            let res = (Self::dp(cur_pos - 1, end_pos, k - 1, memo)
                + Self::dp(cur_pos + 1, end_pos, k - 1, memo))
                % 1000000007;
            memo.insert((cur_pos, k), res);
            res
        }
    }

    pub fn number_of_ways(start_pos: i32, end_pos: i32, k: i32) -> i32 {
        Self::dp(start_pos, end_pos, k, &mut HashMap::new())
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::number_of_ways(1, 2, 3), 3);
        assert_eq!(Solution::number_of_ways(2, 5, 10), 0);
    }
}
