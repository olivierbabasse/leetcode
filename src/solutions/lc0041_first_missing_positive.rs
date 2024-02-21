//! <https://leetcode.com/problems/first-missing-positive/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let len = nums.len();
        for i in 0..len {
            while nums[i] > 0 && nums[i] <= len as i32 && nums[nums[i] as usize - 1] != nums[i] {
                let index = nums[i] as usize - 1;
                nums.swap(i, index);
            }
        }
        for (i, num) in nums.into_iter().enumerate() {
            if num != i as i32 + 1 {
                return i as i32 + 1;
            }
        }
        len as i32 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
        assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
        assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
    }
}
