//! <https://leetcode.com/problems/minimum-cost-to-move-chips-to-the-same-position/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let (mut odds, mut evens) = (0, 0);
        position.into_iter().for_each(|n| {
            if n % 2 == 0 {
                evens += 1;
            } else {
                odds += 1;
            }
        });
        if evens > odds {
            odds
        } else {
            evens
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::min_cost_to_move_chips(vec![1, 2, 3]), 1);
        assert_eq!(Solution::min_cost_to_move_chips(vec![2, 2, 2, 3, 3]), 2);
    }
}
