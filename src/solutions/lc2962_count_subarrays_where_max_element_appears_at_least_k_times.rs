//! <https://leetcode.com/problems/count-subarrays-where-max-element-appears-at-least-k-times/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let max = *nums.iter().max().unwrap();
        let mut max_freq = 0;
        let mut count = 0;
        let mut begin = 0;
        for end in 0..nums.len() {
            if nums[end] == max {
                max_freq += 1;
            }

            while max_freq == k {
                if nums[begin] == max {
                    max_freq -= 1;
                }
                begin += 1;
            }

            count += begin;
        }
        count as i64
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::count_subarrays(vec![1, 3, 2, 3, 3], 2), 6);
        assert_eq!(Solution::count_subarrays(vec![1, 4, 2, 1], 3), 0);
    }
}
