//! <https://leetcode.com/problems/amount-of-time-for-binary-tree-to-be-infected/>

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

type TreeNode = crate::utils::tree::TreeNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    fn tree_to_graph(root: Option<Rc<RefCell<TreeNode>>>) -> HashMap<i32, Vec<i32>> {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut stack = Vec::new();

        if let Some(root_node) = root {
            stack.push(root_node.clone());
        }

        while let Some(node) = stack.pop() {
            if let Some(left) = &node.borrow().left {
                stack.push(left.clone());
                graph
                    .entry(node.borrow().val)
                    .or_default()
                    .push(left.borrow().val);
                graph
                    .entry(left.borrow().val)
                    .or_default()
                    .push(node.borrow().val);
            }

            if let Some(right) = &node.borrow().right {
                stack.push(right.clone());
                graph
                    .entry(node.borrow().val)
                    .or_default()
                    .push(right.borrow().val);
                graph
                    .entry(right.borrow().val)
                    .or_default()
                    .push(node.borrow().val);
            }
        }

        graph
    }

    fn max_depth_from_node(graph: &HashMap<i32, Vec<i32>>, start_node: i32) -> Option<usize> {
        let mut visited = HashSet::new();
        let mut stack = Vec::new();
        let mut max_depth = 0;

        stack.push((start_node, 0));

        while let Some((node, depth)) = stack.pop() {
            if !visited.contains(&node) {
                visited.insert(node);

                if depth > max_depth {
                    max_depth = depth;
                }

                if let Some(adj) = graph.get(&node) {
                    for &n in adj {
                        stack.push((n, depth + 1));
                    }
                }
            }
        }

        if max_depth > 0 {
            Some(max_depth)
        } else {
            None
        }
    }

    pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
        Self::max_depth_from_node(&Self::tree_to_graph(root), start).unwrap_or_default() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::amount_of_time(
                TreeNode::from_vec_opt(&[
                    Some(1),
                    Some(5),
                    Some(3),
                    None,
                    Some(4),
                    Some(10),
                    Some(6),
                    Some(9),
                    Some(2)
                ]),
                3
            ),
            4
        );
        assert_eq!(Solution::amount_of_time(TreeNode::from_vec(&[1]), 1), 0);
    }
}
