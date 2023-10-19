//! <https://leetcode.com/problems/minimum-number-of-operations-to-make-array-continuous/>

struct Solution {}

/// time-complexity : O(n^2)
/// space-complexity : O(1)
impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let n = nums.len() - 1;

        nums.sort();
        nums.dedup();

        let len = nums.len();
        let mut j = 0;
        let mut count = 0;
        let mut max = 0;
        for i in 0..len {
            while j < len && (nums[j] - nums[i]) <= n as i32 {
                count += 1;
                j += 1;
            }
            max = std::cmp::max(max, count);
            count -= 1;
        }

        (n + 1 - max) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::min_operations(vec![4, 2, 5, 3]), 0);
        assert_eq!(Solution::min_operations(vec![1, 2, 3, 5, 6]), 1);
        assert_eq!(Solution::min_operations(vec![1, 10, 100, 1000]), 3);
    }
}
