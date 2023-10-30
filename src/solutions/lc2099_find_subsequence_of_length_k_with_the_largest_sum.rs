//! <https://leetcode.com/problems/find-subsequence-of-length-k-with-the-largest-sum/>

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let len = nums.len();
        let mut sorted_and_indexed = nums.into_iter().enumerate().collect::<Vec<_>>();
        sorted_and_indexed.sort_by_key(|a| a.1);
        let mut res = sorted_and_indexed[len - (k as usize)..len].to_vec();
        res.sort_by_key(|a| a.0);
        res.into_iter().map(|a| a.1).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::max_subsequence(vec![2, 1, 3, 3], 2), vec![3, 3]);
        assert_eq!(
            Solution::max_subsequence(vec![-1, -2, 3, 4], 3),
            vec![-1, 3, 4]
        );
        assert_eq!(Solution::max_subsequence(vec![3, 4, 3, 3], 2), vec![4, 3]);
    }
}
