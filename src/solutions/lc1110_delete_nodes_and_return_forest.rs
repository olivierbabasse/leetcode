//! <https://leetcode.com/problems/delete-nodes-and-return-forest/>

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

type TreeNode = crate::utils::tree::TreeNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    fn traversal(
        root: &Option<Rc<RefCell<TreeNode>>>,
        to_delete: &HashSet<i32>,
        output: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
    ) -> bool {
        if let Some(node) = root {
            let mut node = node.as_ref().borrow_mut();

            if Self::traversal(&node.left, to_delete, output) {
                node.left = None;
            }
            if Self::traversal(&node.right, to_delete, output) {
                node.right = None;
            }

            if to_delete.contains(&node.val) {
                if node.left.is_some() {
                    let mut child = None;
                    std::mem::swap(&mut child, &mut node.left);
                    output.push(child);
                }
                if node.right.is_some() {
                    let mut child = None;
                    std::mem::swap(&mut child, &mut node.right);
                    output.push(child);
                }
                return true;
            }
        }
        false
    }

    pub fn del_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: Vec<i32>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut output = Vec::new();
        let to_delete: HashSet<i32> = to_delete.into_iter().collect();
        if !Self::traversal(&root, &to_delete, &mut output) {
            output.push(root);
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::del_nodes(TreeNode::from_vec(&[1, 2, 3, 4, 5, 6, 7]), vec![3, 5]),
            vec![
                TreeNode::from_vec(&[6]),
                TreeNode::from_vec(&[7]),
                TreeNode::from_vec_opt(&[Some(1), Some(2), None, Some(4)]),
            ]
        );
    }
}
