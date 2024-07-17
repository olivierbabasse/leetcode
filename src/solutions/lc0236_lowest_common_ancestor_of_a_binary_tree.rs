//! <https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/>

use std::cell::RefCell;
use std::rc::Rc;

type TreeNode = crate::utils::tree::TreeNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            if node.borrow().val == p.as_ref().unwrap().borrow().val
                || node.borrow().val == q.as_ref().unwrap().borrow().val
            {
                return Some(node);
            }
            let left =
                Self::lowest_common_ancestor(node.borrow().left.clone(), p.clone(), q.clone());
            let right =
                Self::lowest_common_ancestor(node.borrow().right.clone(), p.clone(), q.clone());
            if left.is_some() && right.is_some() {
                Some(node)
            } else if left.is_none() {
                right
            } else {
                left
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::lowest_common_ancestor(
                TreeNode::from_vec_opt(&[
                    Some(3),
                    Some(5),
                    Some(1),
                    Some(6),
                    Some(2),
                    Some(0),
                    Some(8),
                    None,
                    None,
                    Some(7),
                    Some(4)
                ]),
                TreeNode::from_vec(&[5]),
                TreeNode::from_vec(&[1]),
            )
            .unwrap()
            .borrow()
            .val,
            TreeNode::from_vec(&[3]).unwrap().borrow().val,
        );
        assert_eq!(
            Solution::lowest_common_ancestor(
                TreeNode::from_vec_opt(&[
                    Some(3),
                    Some(5),
                    Some(1),
                    Some(6),
                    Some(2),
                    Some(0),
                    Some(8),
                    None,
                    None,
                    Some(7),
                    Some(4)
                ]),
                TreeNode::from_vec(&[5]),
                TreeNode::from_vec(&[4]),
            )
            .unwrap()
            .borrow()
            .val,
            TreeNode::from_vec(&[5]).unwrap().borrow().val,
        );
    }
}
