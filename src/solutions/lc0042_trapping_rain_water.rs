//! <https://leetcode.com/problems/trapping-rain-water/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let len = height.len();
        let mut maxs_left = vec![0; len];
        let mut maxs_right = vec![0; len];
        let mut max = 0;
        for i in 0..len {
            max = max.max(height[i]);
            maxs_left[i] = max;
        }
        max = 0;
        for i in (0..len).rev() {
            max = max.max(height[i]);
            maxs_right[i] = max;
        }
        let mut water = 0;
        for i in 0..len {
            water += i32::min(maxs_left[i], maxs_right[i]) - height[i];
        }
        water
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
        assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }
}
