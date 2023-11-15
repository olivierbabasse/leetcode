//! <https://leetcode.com/problems/partition-array-according-to-given-pivot/>

use std::cmp::Ordering;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut less = Vec::new();
        let mut pivots = Vec::new();
        let mut more = Vec::new();

        nums.into_iter().for_each(|a| match a.cmp(&pivot) {
            Ordering::Less => less.push(a),
            Ordering::Equal => pivots.push(a),
            Ordering::Greater => more.push(a),
        });

        less.into_iter()
            .chain(pivots)
            .chain(more)
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::pivot_array(vec![9, 12, 5, 10, 14, 3, 10], 10),
            vec![9, 5, 3, 10, 10, 12, 14]
        );
        assert_eq!(
            Solution::pivot_array(vec![-3, 4, 3, 2], 2),
            vec![-3, 2, 4, 3]
        );
    }
}
