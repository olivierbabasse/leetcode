//! <https://leetcode.com/problems/buy-two-chocolates/>

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(1)
impl Solution {
    pub fn buy_choco(mut prices: Vec<i32>, money: i32) -> i32 {
        prices.sort_unstable();
        if prices[0] + prices[1] > money {
            money
        } else {
            money - prices[0] - prices[1]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::buy_choco(vec![1, 2, 2], 3), 0);
        assert_eq!(Solution::buy_choco(vec![3, 2, 3], 3), 3);
        assert_eq!(
            Solution::buy_choco(vec![98, 54, 6, 34, 66, 63, 52, 39], 62),
            22
        );
    }
}
