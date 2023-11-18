//! <https://leetcode.com/problems/frequency-of-the-most-frequent-element/>

struct Solution {}

/// time-complexity : O(n.log(n))
/// space-complexity : O(1)
impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();

        let mut left = 0;
        let mut cur = 0;
        for right in 0..nums.len() {
            let t = nums[right];
            cur += t;
            if (right - left + 1) as i32 * t - cur > k {
                cur -= nums[left];
                left += 1;
            }
        }
        (nums.len() - left) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::max_frequency(vec![1, 2, 4], 5), 3);
        assert_eq!(Solution::max_frequency(vec![1, 4, 8, 13], 5), 2);
        assert_eq!(Solution::max_frequency(vec![3, 6, 9], 2), 1);
    }
}
