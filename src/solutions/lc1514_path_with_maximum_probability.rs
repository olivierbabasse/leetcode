//! <https://leetcode.com/problems/path-with-maximum-probability/>

struct Solution {}

/// time-complexity : O(n*len(edges))
/// space-complexity : O(n)
impl Solution {
    pub fn max_probability(
        n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start_node: i32,
        end_node: i32,
    ) -> f64 {
        let mut maxp = vec![0.0; n as usize];
        maxp[start_node as usize] = 1.0;

        for _ in 0..n {
            for (j, edge) in edges.iter().enumerate() {
                let (u, v) = (edge[0] as usize, edge[1] as usize);
                let p = succ_prob[j];
                if maxp[u] * p > maxp[v] {
                    maxp[v] = maxp[u] * p;
                }
                if maxp[v] * p > maxp[u] {
                    maxp[u] = maxp[v] * p;
                }
            }
        }

        maxp[end_node as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::max_probability(
                3,
                vec![vec![0, 1], vec![1, 2], vec![0, 2]],
                vec![0.5, 0.5, 0.2],
                0,
                2
            ),
            0.25
        );
        assert_eq!(
            Solution::max_probability(
                3,
                vec![vec![0, 1], vec![1, 2], vec![0, 2]],
                vec![0.5, 0.5, 0.3],
                0,
                2
            ),
            0.3
        );
        assert_eq!(
            Solution::max_probability(3, vec![vec![0, 1]], vec![0.5], 0, 2),
            0.0
        );
    }
}
