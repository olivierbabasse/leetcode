//! <https://leetcode.com/problems/number-of-smooth-descent-periods-of-a-stock/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        let mut res = 1;
        let mut begin = 0;
        for end in 1..prices.len() {
            if prices[end - 1] - prices[end] != 1 {
                begin = end;
            }
            res += end - begin + 1;
        }

        res as i64
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::get_descent_periods(vec![3, 2, 1, 4]), 7);
        assert_eq!(Solution::get_descent_periods(vec![8, 6, 7, 7]), 4);
        assert_eq!(Solution::get_descent_periods(vec![1]), 1);
    }
}
