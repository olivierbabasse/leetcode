//! <https://leetcode.com/problems/richest-customer-wealth/>

struct Solution {}

/// time-complexity : O(accounts * account_len)
/// space-complexity : O(1)
impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts.iter().map(|v| v.iter().sum()).max().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1]]),
            6
        );
        assert_eq!(
            Solution::maximum_wealth(vec![vec![1, 5], vec![7, 3], vec![3, 5]]),
            10
        );
        assert_eq!(
            Solution::maximum_wealth(vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]]),
            17
        );
    }
}
