//! <https://leetcode.com/problems/delete-leaves-with-a-given-value/>

use std::cell::RefCell;
use std::rc::Rc;

type TreeNode = crate::utils::tree::TreeNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(log(n))
impl Solution {
    pub fn remove_leaf_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(r) = root {
            let mut root = r.borrow_mut();

            root.left = Self::remove_leaf_nodes(root.left.take(), target);
            root.right = Self::remove_leaf_nodes(root.right.take(), target);

            if root.left.is_none() && root.right.is_none() && root.val == target {
                return None;
            }

            return Some(r.clone());
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::remove_leaf_nodes(
                TreeNode::from_vec_opt(&[
                    Some(1),
                    Some(2),
                    Some(3),
                    Some(2),
                    None,
                    Some(2),
                    Some(4)
                ]),
                2
            ),
            TreeNode::from_vec_opt(&[Some(1), None, Some(3), None, Some(4)])
        );
    }
}
