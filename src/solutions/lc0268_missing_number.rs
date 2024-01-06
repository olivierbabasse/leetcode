//! <https://leetcode.com/problems/missing-number/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        ((n * (n + 1)) / 2) as i32 - nums.into_iter().sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2);
        assert_eq!(Solution::missing_number(vec![0, 1]), 2);
        assert_eq!(Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }
}
