//! <https://leetcode.com/problems/subarray-product-less-than-k/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k <= 1 {
            return 0;
        }

        let (mut res, mut cur) = (0, 1);
        let (mut begin, mut end) = (0, 0);

        while end < nums.len() {
            cur *= nums[end];

            while cur >= k {
                cur /= nums[begin];
                begin += 1;
            }

            res += end - begin + 1;

            end += 1;
        }

        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100),
            8
        );
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![1, 2, 3], 0),
            0
        );
    }
}
