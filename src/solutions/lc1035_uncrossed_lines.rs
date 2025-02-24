// <https://leetcode.com/problems/uncrossed-lines/>

struct Solution {}

/// time-complexity : O(len(nums1)*len(nums2))
/// space-complexity : O(len(nums1)*len(nums2))
impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (len1, len2) = (nums1.len(), nums2.len());
        let mut dp = vec![vec![0; len2 + 1]; len1 + 1];
        for i in 1..=len1 {
            for j in 1..=len2 {
                dp[i][j] = if nums1[i - 1] == nums2[j - 1] {
                    dp[i - 1][j - 1] + 1
                } else {
                    dp[i][j - 1].max(dp[i - 1][j])
                }
            }
        }
        dp[len1][len2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::max_uncrossed_lines(vec![1, 4, 2], vec![1, 2, 4]),
            2
        );
        assert_eq!(
            Solution::max_uncrossed_lines(vec![2, 5, 1, 2, 5], vec![10, 5, 2, 1, 5, 2]),
            3
        );
        assert_eq!(
            Solution::max_uncrossed_lines(vec![1, 3, 7, 1, 7, 5], vec![1, 9, 2, 5, 1]),
            2
        );
    }
}
