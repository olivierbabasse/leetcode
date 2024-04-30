//! <https://leetcode.com/problems/minimum-number-of-operations-to-make-array-xor-equal-to-k/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let x = nums.into_iter().fold(0, |x, n| x ^ n);
        (x ^ k).count_ones() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::min_operations(vec![2, 1, 3, 4], 1), 2);
        assert_eq!(Solution::min_operations(vec![2, 0, 2, 0], 0), 0);
    }
}
