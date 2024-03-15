//! <https://leetcode.com/problems/maximum-product-subarray/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let (mut res, mut imin, mut imax) = (nums[0], 1, 1);
        for n in nums.into_iter() {
            if n < 0 {
                std::mem::swap(&mut imin, &mut imax);
            }
            imax = n.max(imax * n);
            imin = n.min(imin * n);
            res = res.max(imax);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::max_product(vec![2, 3, -2, 4]), 6);
        assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);
        assert_eq!(Solution::max_product(vec![-2]), -2);
    }
}
