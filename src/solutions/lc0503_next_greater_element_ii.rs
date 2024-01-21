//! <https://leetcode.com/problems/next-greater-element-ii/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::<i32>::new();
        let mut nge = HashMap::new();

        let len = nums.len();
        for index in (0..2 * len).rev() {
            let n = nums[index % len];
            while !stack.is_empty() && stack.last().unwrap() <= &n {
                stack.pop();
            }
            nge.insert(index, *stack.last().unwrap_or(&-1));
            stack.push(n);
        }

        nums.into_iter()
            .enumerate()
            .map(|(index, _)| *nge.get(&index).unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::next_greater_elements(vec![1, 2, 1]),
            vec![2, -1, 2]
        );
        assert_eq!(
            Solution::next_greater_elements(vec![1, 2, 3, 4, 3]),
            [2, 3, 4, -1, 4]
        );
    }
}
