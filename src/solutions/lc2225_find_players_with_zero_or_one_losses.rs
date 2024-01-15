//! <https://leetcode.com/problems/find-players-with-zero-or-one-losses/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut losses: HashMap<i32, i32> = HashMap::new();
        for m in matches {
            losses.entry(m[0]).or_default();
            *losses.entry(m[1]).or_default() += 1;
        }
        let mut lose0 = losses
            .iter()
            .filter(|(_, &v)| v == 0)
            .map(|(&k, _)| k)
            .collect::<Vec<_>>();
        lose0.sort_unstable();
        let mut lose1 = losses
            .iter()
            .filter(|(_, &v)| v == 1)
            .map(|(&k, _)| k)
            .collect::<Vec<_>>();
        lose1.sort_unstable();
        vec![lose0, lose1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::find_winners(vec![
                vec![1, 3],
                vec![2, 3],
                vec![3, 6],
                vec![5, 6],
                vec![5, 7],
                vec![4, 5],
                vec![4, 8],
                vec![4, 9],
                vec![10, 4],
                vec![10, 9]
            ]),
            vec![vec![1, 2, 10], vec![4, 5, 7, 8]]
        );
    }
}
