//! <https://leetcode.com/problems/next-greater-element-i/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::<i32>::new();
        let mut nge = HashMap::new();

        for n in nums2.into_iter().rev() {
            while !stack.is_empty() && stack.last().unwrap() < &n {
                stack.pop();
            }
            nge.insert(n, *stack.last().unwrap_or(&-1));
            stack.push(n);
        }

        nums1.into_iter().map(|n| *nge.get(&n).unwrap()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]),
            vec![-1, 3, -1]
        );
        assert_eq!(
            Solution::next_greater_element(vec![2, 4], vec![1, 2, 3, 4]),
            vec![3, -1]
        );
    }
}
