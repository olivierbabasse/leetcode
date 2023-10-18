//! <https://leetcode.com/problems/parallel-courses-iii/>

use std::collections::HashMap;

struct Solution {}

impl Solution {
    /// time-complexity : O(n)
    /// space-complexity : O(n)
    fn dfs(
        cur_node: usize,
        time: &Vec<i32>,
        adj: &Vec<Vec<usize>>,
        memo: &mut HashMap<usize, i32>,
    ) -> i32 {
        if let Some(t) = memo.get(&cur_node) {
            *t
        } else {
            let t = time[cur_node]
                + adj[cur_node].iter().fold(0, |acc, child_node| {
                    acc.max(Self::dfs(*child_node, time, adj, memo))
                });
            memo.insert(cur_node, t);
            t
        }
    }

    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        let mut memo = HashMap::new();
        let n = n as usize;
        let mut adj = vec![Vec::new(); n];
        let mut has_parents = vec![false; n];

        for r in relations {
            adj[(r[1] as usize) - 1].push((r[0] as usize) - 1);
            has_parents[(r[0] as usize) - 1] = true;
        }

        let root_nodes = has_parents
            .into_iter()
            .enumerate()
            .filter(|(_, has)| !*has)
            .map(|(n, _)| n)
            .collect::<Vec<_>>();

        root_nodes.iter().fold(0, |acc, root_node| {
            acc.max(Self::dfs(*root_node, &time, &adj, &mut memo))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::minimum_time(3, vec![vec![1, 3], vec![2, 3]], vec![3, 2, 5]),
            8
        );
        assert_eq!(
            Solution::minimum_time(
                5,
                vec![vec![1, 5], vec![2, 5], vec![3, 5], vec![3, 4], vec![4, 5]],
                vec![1, 2, 3, 4, 5]
            ),
            12
        );
        assert_eq!(
            Solution::minimum_time(2, vec![vec![2, 1]], vec![10000, 10000]),
            20000
        );
        assert_eq!(Solution::minimum_time(2, vec![], vec![3, 5]), 5);
    }
}
