//! <https://leetcode.com/problems/best-time-to-buy-and-sell-stock/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        let mut max_profit = 0;
        for p in prices {
            if p < min {
                min = p;
            } else {
                let profit = p - min;
                max_profit = max_profit.max(profit);
            }
        }
        max_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
