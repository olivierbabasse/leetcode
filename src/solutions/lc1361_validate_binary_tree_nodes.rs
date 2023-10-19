//! <https://leetcode.com/problems/validate-binary-tree-nodes/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    fn visit(
        index: usize,
        left_child: &Vec<i32>,
        right_child: &Vec<i32>,
        visited: &mut Vec<bool>,
    ) -> bool {
        if visited[index] {
            return false;
        }

        visited[index] = true;

        if left_child[index] != -1
            && !Self::visit(left_child[index] as usize, left_child, right_child, visited)
        {
            return false;
        }

        if right_child[index] != -1
            && !Self::visit(
                right_child[index] as usize,
                left_child,
                right_child,
                visited,
            )
        {
            return false;
        }

        true
    }

    pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
        let mut ins = vec![0; n as usize];
        left_child.iter().chain(right_child.iter()).for_each(|c| {
            if *c != -1 {
                ins[*c as usize] += 1
            }
        });
        let roots = ins
            .into_iter()
            .enumerate()
            .filter(|(_, v)| *v == 0)
            .map(|(c, _)| c)
            .collect::<Vec<_>>();
        if roots.len() != 1 {
            return false;
        }

        let mut visited = vec![false; n as usize];
        Self::visit(roots[0], &left_child, &right_child, &mut visited)
            && visited.into_iter().all(|v| v)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert!(Solution::validate_binary_tree_nodes(
            4,
            vec![1, -1, 3, -1],
            vec![2, -1, -1, -1]
        ));
        assert!(!Solution::validate_binary_tree_nodes(
            4,
            vec![1, -1, 3, -1],
            vec![2, 3, -1, -1]
        ));
        assert!(!Solution::validate_binary_tree_nodes(
            2,
            vec![1, 0],
            vec![-1, -1]
        ));
        assert!(!Solution::validate_binary_tree_nodes(
            6,
            vec![1, -1, -1, 4, -1, -1],
            vec![2, -1, -1, 5, -1, -1]
        ));
        assert!(Solution::validate_binary_tree_nodes(
            4,
            vec![3, -1, 1, -1],
            vec![-1, -1, 0, -1]
        ));
    }
}
