//! <https://leetcode.com/problems/product-of-array-except-self/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut prefix = vec![1; n];
        let mut suffix = vec![1; n];
        let mut res = vec![0; n];

        for i in 1..n {
            prefix[i] = prefix[i - 1] * nums[i - 1];
        }
        for i in (0..=n - 2).rev() {
            suffix[i] = suffix[i + 1] * nums[i + 1];
        }
        for i in 0..n {
            res[i] = prefix[i] * suffix[i];
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
        assert_eq!(
            Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }
}
