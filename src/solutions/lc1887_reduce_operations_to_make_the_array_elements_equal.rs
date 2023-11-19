//! <https://leetcode.com/problems/reduction-operations-to-make-the-array-elements-equal/>

struct Solution {}

/// time-complexity : O(n.log(n))
/// space-complexity : O(1)
impl Solution {
    pub fn reduction_operations(mut nums: Vec<i32>) -> i32 {
        let mut total = 0;
        let mut steps = 0;
        nums.sort_unstable();
        for i in 1..nums.len() {
            if nums[i - 1] != nums[i] {
                steps += 1;
            }
            total += steps;
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::reduction_operations(vec![5, 1, 3]), 3);
        assert_eq!(Solution::reduction_operations(vec![1, 1, 1]), 0);
        assert_eq!(Solution::reduction_operations(vec![1, 1, 2, 2, 3]), 4);
    }
}
