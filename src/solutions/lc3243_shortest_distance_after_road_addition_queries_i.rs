//! <https://leetcode.com/problems/shortest-distance-after-road-addition-queries-i/>

use std::collections::VecDeque;

struct Solution {}

/// time-complexity : O(q*(n+q))
/// space-complexity : O(n+q)
impl Solution {
    fn bfs(adj: &[Vec<usize>]) -> i32 {
        let mut visited = vec![false; adj.len()];
        let mut queue = VecDeque::new();
        let mut count = 0;

        queue.push_front(0);
        visited[0] = true;

        while !queue.is_empty() {
            let len = queue.len();
            for _ in 0..len {
                let current_node = queue.pop_front().unwrap();
                if current_node == adj.len() - 1 {
                    return count;
                }

                for &n in &adj[current_node] {
                    if !visited[n] {
                        queue.push_back(n);
                        visited[n] = true;
                    }
                }
            }
            count += 1;
        }

        -1
    }

    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut res = Vec::new();

        let mut adj = vec![Vec::new(); n];
        for (i, a) in adj.iter_mut().enumerate().take(n - 1) {
            a.push(i + 1);
        }

        for q in queries {
            adj[q[0] as usize].push(q[1] as usize);
            res.push(Self::bfs(&adj));
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
            Solution::shortest_distance_after_queries(5, vec![vec![2, 4], vec![0, 2], vec![0, 4]]),
            vec![3, 2, 1]
        );
        assert_eq!(
            Solution::shortest_distance_after_queries(4, vec![vec![0, 3], vec![0, 2]]),
            vec![1, 1]
        );
    }
}
