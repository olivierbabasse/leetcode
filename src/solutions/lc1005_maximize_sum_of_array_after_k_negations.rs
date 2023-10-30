//! <https://leetcode.com/problems/maximize-sum-of-array-after-k-negations/>

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(1)
impl Solution {
    pub fn largest_sum_after_k_negations(mut nums: Vec<i32>, mut k: i32) -> i32 {
        nums.sort_unstable();
        for elem in &mut nums {
            if *elem < 0 && k > 0 {
                *elem = -*elem;
                k -= 1;
            }
        }
        if k % 2 != 0 {
            nums.sort_unstable();
            nums[0] = -nums[0];
        }
        nums.into_iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::largest_sum_after_k_negations(vec![4, 2, 3], 1), 5);
        assert_eq!(
            Solution::largest_sum_after_k_negations(vec![3, -1, 0, 2], 3),
            6
        );
        assert_eq!(
            Solution::largest_sum_after_k_negations(vec![2, -3, -1, 5, -4], 2),
            13
        );
    }
}
