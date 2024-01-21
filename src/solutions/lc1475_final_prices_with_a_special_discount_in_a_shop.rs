//! <https://leetcode.com/problems/final-prices-with-a-special-discount-in-a-shop/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::<i32>::new();
        let mut res = Vec::new();

        for n in prices.into_iter().rev() {
            while !stack.is_empty() && stack.last().unwrap() > &n {
                stack.pop();
            }
            res.push(n - *stack.last().unwrap_or(&0));
            stack.push(n);
        }

        res.reverse();
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::final_prices(vec![8, 4, 6, 2, 3]),
            vec![4, 2, 4, 2, 3]
        );
        assert_eq!(
            Solution::final_prices(vec![1, 2, 3, 4, 5]),
            vec![1, 2, 3, 4, 5]
        );
        assert_eq!(Solution::final_prices(vec![10, 1, 1, 6]), vec![9, 0, 1, 6]);
    }
}
