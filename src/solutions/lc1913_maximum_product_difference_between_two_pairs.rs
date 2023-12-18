//! <https://leetcode.com/problems/maximum-product-difference-between-two-pairs/>

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(1)
impl Solution {
    pub fn max_product_difference(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        nums.sort_unstable();
        nums[n - 1] * nums[n - 2] - nums[1] * nums[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::max_product_difference(vec![5, 6, 2, 7, 4]), 34);
        assert_eq!(
            Solution::max_product_difference(vec![4, 2, 5, 9, 7, 4, 8]),
            64
        );
    }
}
