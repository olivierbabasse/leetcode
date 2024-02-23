//! <https://leetcode.com/problems/all-paths-from-source-to-target/>

struct Solution {}

/// time-complexity : O()
/// space-complexity : O()
impl Solution {
    fn rec(graph: &Vec<Vec<i32>>, node: i32, cur_path: &mut Vec<i32>, paths: &mut Vec<Vec<i32>>) {
        if node == graph.len() as i32 - 1 {
            paths.push(cur_path.clone());
            return;
        }

        for &v in &graph[node as usize] {
            cur_path.push(v);
            Self::rec(graph, v, cur_path, paths);
            cur_path.pop();
        }
    }

    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut paths = Vec::new();
        Self::rec(&graph, 0, &mut vec![0], &mut paths);
        paths
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::all_paths_source_target(vec![vec![1, 2], vec![3], vec![3], vec![]]),
            vec![vec![0, 1, 3], vec![0, 2, 3]]
        );
        assert_eq!(
            Solution::all_paths_source_target(vec![
                vec![4, 3, 1],
                vec![3, 2, 4],
                vec![3],
                vec![4],
                vec![]
            ]),
            vec![
                vec![0, 4],
                vec![0, 3, 4],
                vec![0, 1, 3, 4],
                vec![0, 1, 2, 3, 4],
                vec![0, 1, 4]
            ]
        );
    }
}
