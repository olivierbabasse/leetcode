//! <https://leetcode.com/problems/restore-the-array-from-adjacent-pairs/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let mut adj_lists: HashMap<i32, Vec<i32>> = HashMap::new();
        adjacent_pairs.iter().for_each(|pair| {
            adj_lists.entry(pair[0]).or_default().push(pair[1]);
            adj_lists.entry(pair[1]).or_default().push(pair[0]);
        });

        let mut cur = *adj_lists
            .iter()
            .find(|(k, _v)| adj_lists.get(k).unwrap().len() == 1)
            .map(|(k, _)| k)
            .unwrap();
        let mut next = adj_lists.get(&cur).unwrap()[0];

        let mut res = vec![cur];
        loop {
            res.push(next);
            if let Some(&n) = adj_lists
                .get(&next)
                .unwrap()
                .iter()
                .find(|&item| item != &cur)
            {
                cur = next;
                next = n;
            } else {
                break;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_cases() {
        assert!(utils::array_eq(
            &Solution::restore_array(vec![vec![2, 1], vec![3, 4], vec![3, 2]]),
            &[1, 2, 3, 4]
        ));
        assert!(utils::array_eq(
            &Solution::restore_array(vec![vec![4, -2], vec![1, 4], vec![-3, 1]]),
            &[-2, 4, 1, -3]
        ));
        assert!(utils::array_eq(
            &Solution::restore_array(vec![vec![100000, -100000]]),
            &[100000, -100000]
        ));
    }
}
