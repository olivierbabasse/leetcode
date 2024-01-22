//! <https://leetcode.com/problems/set-mismatch/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn find_error_nums(mut nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();

        let mut duplicate = 0;
        for i in 0..len {
            let index = (nums[i].abs() - 1) as usize;
            if nums[index] > 0 {
                nums[index] = -nums[index];
            } else {
                duplicate = nums[i].abs();
            }
        }

        let mut missing = 0;
        for (i, &num) in nums.iter().enumerate() {
            if num > 0 {
                missing = i + 1;
            }
        }

        vec![duplicate, missing as i32]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::find_error_nums(vec![1, 2, 2, 4]), vec![2, 3]);
        assert_eq!(Solution::find_error_nums(vec![1, 1]), vec![1, 2]);
    }
}
