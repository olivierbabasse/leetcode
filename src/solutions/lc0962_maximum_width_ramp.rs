//! <https://leetcode.com/problems/maximum-width-ramp/>

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
        let mut indices = (0..nums.len()).collect::<Vec<_>>();
        indices.sort_by(|a, b| nums[*a].cmp(&nums[*b]));
        let (mut min, mut max) = (indices.len() as i32, 0i32);
        for i in indices {
            max = max.max(i as i32 - min);
            min = min.min(i as i32);
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::max_width_ramp(vec![6, 0, 8, 2, 1, 5]), 4);
        assert_eq!(
            Solution::max_width_ramp(vec![9, 8, 1, 0, 1, 9, 4, 0, 4, 1]),
            7
        );
    }
}
