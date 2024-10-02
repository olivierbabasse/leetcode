//! <https://leetcode.com/problems/rank-transform-of-an-array/>

use std::collections::BTreeMap;

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let mut ranks = arr.iter().map(|&a| (a, 0)).collect::<BTreeMap<i32, i32>>();
        for (rank, (_, r)) in ranks.iter_mut().enumerate() {
            *r = 1 + rank as i32;
        }
        arr.into_iter()
            .map(|a| *ranks.get(&a).unwrap_or(&0))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::array_rank_transform(vec![40, 10, 20, 30]),
            vec![4, 1, 2, 3]
        );
        assert_eq!(
            Solution::array_rank_transform(vec![100, 100, 100]),
            vec![1, 1, 1]
        );
        assert_eq!(
            Solution::array_rank_transform(vec![37, 12, 28, 9, 100, 56, 80, 5, 12]),
            vec![5, 3, 4, 2, 8, 6, 7, 1, 3]
        );
    }
}
