//! <https://leetcode.com/problems/sum-of-absolute-differences-in-a-sorted-array/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut prefix_sum = vec![nums[0]; n];
        let mut postfix_sum = vec![nums[n - 1]; n];
        for i in 1..n {
            prefix_sum[i] = prefix_sum[i - 1] + nums[i];
            postfix_sum[n - i - 1] = postfix_sum[n - i] + nums[n - i - 1];
        }

        nums.into_iter()
            .enumerate()
            .map(|(i, num)| {
                num * (2 * i as i32 - n as i32 + 1) - prefix_sum[i] + postfix_sum[i] as i32
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::get_sum_absolute_differences(vec![2, 3, 5]),
            vec![4, 3, 5]
        );
        assert_eq!(
            Solution::get_sum_absolute_differences(vec![1, 4, 6, 8, 10]),
            vec![24, 15, 13, 15, 21]
        );
    }
}
