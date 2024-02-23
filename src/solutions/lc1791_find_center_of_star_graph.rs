//! <https://leetcode.com/problems/find-center-of-star-graph/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        let mut adj: HashMap<i32, Vec<i32>> = HashMap::new();
        for e in edges {
            adj.entry(e[0]).or_default().push(e[1]);
            adj.entry(e[1]).or_default().push(e[0]);
        }
        for (k, v) in adj {
            if v.len() > 1 {
                return k;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::find_center(vec![vec![1, 2], vec![2, 3], vec![4, 2]]),
            2
        );
        assert_eq!(
            Solution::find_center(vec![vec![1, 2], vec![5, 1], vec![1, 3], vec![1, 4]]),
            1
        );
    }
}
