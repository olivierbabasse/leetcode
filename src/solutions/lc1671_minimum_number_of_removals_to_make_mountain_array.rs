//! <https://leetcode.com/problems/minimum-number-of-removals-to-make-mountain-array/>

struct Solution {}

/// time-complexity : O(n^2)
/// space-complexity : O(n)
impl Solution {
    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let (mut lis, mut lds) = (vec![1; len], vec![1; len]);

        for i in 0..len {
            for j in (0..=i.saturating_sub(1)).rev() {
                if nums[i] > nums[j] {
                    lis[i] = lis[i].max(lis[j] + 1);
                }
            }
        }

        for i in (0..=len.saturating_sub(1)).rev() {
            for j in i + 1..len {
                if nums[i] > nums[j] {
                    lds[i] = lds[i].max(lds[j] + 1)
                }
            }
        }

        (0..len)
            .filter(|&i| lis[i] > 1 && lds[i] > 1)
            .map(|i| len as i32 - lis[i] - lds[i] + 1)
            .min()
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::minimum_mountain_removals(vec![1, 3, 1]), 0);
        assert_eq!(
            Solution::minimum_mountain_removals(vec![2, 1, 1, 5, 6, 2, 3, 1]),
            3
        );
    }
}
