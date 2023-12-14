//! <https://leetcode.com/problems/maximum-product-of-two-elements-in-an-array/>

struct Solution {}

/// time-complexity : O(1)
/// space-complexity : O(n)
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max1 = 0;
        let mut max2 = 0;
        nums.into_iter().for_each(|cur| {
            if cur > max1 {
                max2 = max1;
                max1 = cur;
            } else if cur > max2 {
                max2 = cur;
            }
        });
        (max1 - 1) * (max2 - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::max_product(vec![3, 4, 5, 2]), 12);
        assert_eq!(Solution::max_product(vec![1, 5, 4, 5]), 16);
        assert_eq!(Solution::max_product(vec![3, 7]), 12);
    }
}
