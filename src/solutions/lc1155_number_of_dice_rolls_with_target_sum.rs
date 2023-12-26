//! <https://leetcode.com/problems/number-of-dice-rolls-with-target-sum/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(n*target)
/// space-complexity : O(n*target)
impl Solution {
    fn rec(n: i32, k: i32, target: i32, cache: &mut HashMap<(i32, i32), i32>) -> i32 {
        if n <= 0 {
            if target == 0 {
                1
            } else {
                0
            }
        } else if let Some(&val) = cache.get(&(n, target)) {
            val
        } else {
            let val = (1..=k).fold(0, |acc, i| {
                (acc + Self::rec(n - 1, k, target - i, cache)) % 1000000007
            });
            cache.insert((n, target), val);
            val
        }
    }

    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        let mut cache = HashMap::new();
        (1..=k).fold(0, |acc, i| {
            (acc + Self::rec(n - 1, k, target - i, &mut cache)) % 1000000007
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::num_rolls_to_target(1, 6, 3), 1);
        assert_eq!(Solution::num_rolls_to_target(2, 6, 7), 6);
        assert_eq!(Solution::num_rolls_to_target(30, 30, 500), 222616187);
    }
}
