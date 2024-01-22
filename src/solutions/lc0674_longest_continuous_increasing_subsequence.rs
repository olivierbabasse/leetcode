//! <https://leetcode.com/problems/longest-continuous-increasing-subsequence/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let mut last = i32::MIN;
        let mut count = 0;
        let mut max_count = 0;
        for n in nums {
            if n > last {
                count += 1;
            } else {
                count = 1;
            }
            last = n;
            max_count = i32::max(max_count, count);
        }
        max_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::find_length_of_lcis(vec![1, 3, 5, 4, 7]), 3);
        assert_eq!(Solution::find_length_of_lcis(vec![2, 2, 2, 2, 2]), 1);
    }
}
