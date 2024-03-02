//! <https://leetcode.com/problems/squares-of-a-sorted-array/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut j = nums.partition_point(|&n| n < 0) as i32;
        let mut i = j - 1;
        while i >= 0 || j < nums.len() as i32 {
            if i >= 0 && j < nums.len() as i32 {
                if nums[i as usize].abs() < nums[j as usize].abs() {
                    res.push(nums[i as usize].pow(2));
                    i -= 1;
                } else {
                    res.push(nums[j as usize].pow(2));
                    j += 1;
                }
            } else if i >= 0 {
                res.push(nums[i as usize].pow(2));
                i -= 1;
            } else {
                res.push(nums[j as usize].pow(2));
                j += 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
            vec![0, 1, 9, 16, 100]
        );
        assert_eq!(
            Solution::sorted_squares(vec![-7, -3, 2, 3, 11]),
            vec![4, 9, 9, 49, 121]
        );
    }
}
