//! <https://leetcode.com/problems/maximum-difference-between-increasing-elements/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        let mut max_diff = -1;
        for n in nums {
            if n < min {
                min = n;
            } else {
                let diff = n - min;
                max_diff = max_diff.max(diff);
            }
        }
        if max_diff > 0 {
            max_diff
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::maximum_difference(vec![7, 1, 5, 4]), 4);
        assert_eq!(Solution::maximum_difference(vec![9, 4, 3, 2]), -1);
        assert_eq!(Solution::maximum_difference(vec![1, 5, 2, 10]), 9);
    }
}
