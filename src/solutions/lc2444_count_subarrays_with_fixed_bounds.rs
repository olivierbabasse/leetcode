//! <https://leetcode.com/problems/count-subarrays-with-fixed-bounds/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut count = 0;
        let (mut min_index, mut max_index, mut last_before) = (-1, -1, -1);
        for i in 0..nums.len() as i64 {
            let n = nums[i as usize];
            if n < min_k || n > max_k {
                last_before = i;
            }
            if n == min_k {
                min_index = i;
            }
            if n == max_k {
                max_index = i;
            }
            let arrays = min_index.min(max_index) - last_before;
            if arrays > 0 {
                count += arrays;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::count_subarrays(vec![1, 3, 5, 2, 7, 5], 1, 5), 2);
        assert_eq!(Solution::count_subarrays(vec![1, 1, 1, 1], 1, 1), 10);
    }
}
