//! <https://leetcode.com/problems/minimum-operations-to-collect-elements/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let len = nums.len();
        let mut found = vec![false; len + 1];
        let mut found_count = 0;
        let mut ops = 0;

        for index in (0..len).rev() {
            ops += 1;
            if nums[index] <= k && !found[nums[index] as usize] {
                found[nums[index] as usize] = true;
                found_count += 1;
            }
            if found_count == k {
                break;
            }
        }

        ops
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::min_operations(vec![3, 1, 5, 4, 2], 2), 4);
        assert_eq!(Solution::min_operations(vec![3, 1, 5, 4, 2], 5), 5);
        assert_eq!(Solution::min_operations(vec![3, 2, 5, 3, 1], 3), 4);
    }
}
