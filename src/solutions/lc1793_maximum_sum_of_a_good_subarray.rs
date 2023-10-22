//! <https://leetcode.com/problems/maximum-score-of-a-good-subarray/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let len = nums.len();
        let mut i = k;
        let mut j = k;
        let mut min_value = nums[k];
        let mut max_result = nums[k];

        while i > 0 || j < len - 1 {
            if i > 0 && nums[i - 1] >= min_value {
                i -= 1;
            } else if j < len - 1 && nums[j + 1] >= min_value {
                j += 1;
            } else if i > 0 && j < len - 1 {
                if nums[i - 1] > nums[j + 1] {
                    i -= 1;
                    min_value = nums[i];
                } else {
                    j += 1;
                    min_value = nums[j];
                }
            } else if i > 0 {
                i -= 1;
                min_value = nums[i];
            } else {
                j += 1;
                min_value = nums[j];
            }

            max_result = max_result.max(min_value * (j - i + 1) as i32);
        }

        max_result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::maximum_score(vec![1, 4, 3, 7, 4, 5], 3), 15);
        assert_eq!(Solution::maximum_score(vec![5, 5, 4, 5, 4, 1, 1, 1], 0), 20);
    }
}
