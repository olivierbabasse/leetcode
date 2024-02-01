//! <https://leetcode.com/problems/divide-array-into-arrays-with-max-difference/>

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        nums.sort_unstable();
        for i in (0..nums.len()).step_by(3) {
            if nums[i + 2] - nums[i] <= k {
                res.push(vec![nums[i], nums[i + 1], nums[i + 2]]);
            } else {
                return vec![];
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
            Solution::divide_array(vec![1, 3, 4, 8, 7, 9, 3, 5, 1], 2),
            vec![vec![1, 1, 3], vec![3, 4, 5], vec![7, 8, 9]]
        );
        assert_eq!(
            Solution::divide_array(vec![1, 3, 3, 2, 7, 3], 3),
            Vec::<Vec<i32>>::new()
        );
    }
}
