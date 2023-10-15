//! <https://leetcode.com/problems/number-of-ways-to-stay-in-the-same-place-after-some-steps/>

use std::collections::HashMap;

struct Solution {}

impl Solution {
    /// time-complexity : O(steps * arraylen)
    /// space-complexity : O(steps * arraylen)
    fn dp(pos: i32, steps: i32, arr_len: i32, memo: &mut HashMap<(i32, i32), i32>) -> i32 {
        if steps == 0 {
            if pos == 0 {
                1
            } else {
                0
            }
        } else if let Some(res) = memo.get(&(pos, steps)) {
            *res
        } else {
            let mut res = Self::dp(pos, steps - 1, arr_len, memo) % 1000000007;
            if pos > 0 {
                res = (res + Self::dp(pos - 1, steps - 1, arr_len, memo)) % 1000000007;
            }
            if pos < arr_len - 1 {
                res = (res + Self::dp(pos + 1, steps - 1, arr_len, memo)) % 1000000007;
            }
            let res = res % 1000000007;
            memo.insert((pos, steps), res);
            res
        }
    }

    pub fn num_ways(steps: i32, arr_len: i32) -> i32 {
        Self::dp(0, steps, arr_len, &mut HashMap::new())
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::num_ways(3, 2), 4);
        assert_eq!(Solution::num_ways(2, 4), 2);
        assert_eq!(Solution::num_ways(4, 2), 8);
        assert_eq!(Solution::num_ways(27, 7), 127784505);
    }
}
