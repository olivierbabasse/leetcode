//! <https://leetcode.com/problems/add-one-row-to-tree/>

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

type TreeNode = crate::utils::tree::TreeNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    fn rec(node: &Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32, cur_depth: i32) {
        if let Some(node) = node {
            if depth == cur_depth + 1 {
                let mut node = node.deref().borrow_mut();
                node.left = Some(Rc::new(RefCell::new(TreeNode {
                    val,
                    left: node.left.take(),
                    right: None,
                })));
                node.right = Some(Rc::new(RefCell::new(TreeNode {
                    val,
                    left: None,
                    right: node.right.take(),
                })));
            } else {
                Self::rec(&node.deref().borrow().left, val, depth, cur_depth + 1);
                Self::rec(&node.deref().borrow().right, val, depth, cur_depth + 1);
            }
        }
    }

    pub fn add_one_row(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        depth: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if depth == 1 {
            Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: root,
                right: None,
            })))
        } else {
            Self::rec(&root, val, depth, 1);
            root
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Solution, TreeNode};

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::add_one_row(TreeNode::from_vec(&[4, 2, 6, 3, 1, 5]), 1, 2),
            TreeNode::from_vec_opt(&[
                Some(4),
                Some(1),
                Some(1),
                Some(2),
                None,
                None,
                Some(6),
                Some(3),
                Some(1),
                Some(5)
            ])
        );
    }
}
