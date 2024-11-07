//! <https://leetcode.com/problems/find-if-array-can-be-sorted/>

struct Solution {}

/// time-complexity : O(n^2)
/// space-complexity : O(n)
impl Solution {
    pub fn can_sort_array(mut nums: Vec<i32>) -> bool {
        let len = nums.len();
        for i in 0..len {
            for j in 0..len - i - 1 {
                if nums[j] > nums[j + 1] {
                    if nums[j].count_ones() == nums[j + 1].count_ones() {
                        nums.swap(j, j + 1);
                    } else {
                        return false;
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert!(Solution::can_sort_array(vec![8, 4, 2, 30, 15]));
        assert!(Solution::can_sort_array(vec![1, 2, 3, 4, 5]));
        assert!(!Solution::can_sort_array(vec![3, 16, 8, 4, 2]));
    }
}
