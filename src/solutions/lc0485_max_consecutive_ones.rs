//! <https://leetcode.com/problems/max-consecutive-ones/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut max_count = 0;
        let mut cur_count = 0;
        for num in nums {
            if num == 1 {
                cur_count += 1;
                max_count = max_count.max(cur_count);
            } else {
                cur_count = 0;
            }
        }
        max_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]),
            3
        );
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]),
            2
        );
        assert_eq!(Solution::find_max_consecutive_ones(vec![0]), 0);
        assert_eq!(Solution::find_max_consecutive_ones(vec![1]), 1);
        assert_eq!(Solution::find_max_consecutive_ones(vec![0, 0]), 0);
    }
}
