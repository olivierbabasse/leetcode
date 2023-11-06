//! <https://leetcode.com/problems/insufficient-nodes-in-root-to-leaf-paths/>

use std::cell::RefCell;
use std::rc::Rc;

type TreeNode = crate::utils::tree::TreeNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    fn traversal(root: Option<Rc<RefCell<TreeNode>>>, limit: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(ref r) = root {
            let val = r.borrow().val;
            let mut left = r.borrow_mut().left.take();
            let mut right = r.borrow_mut().right.take();

            if left.is_none() && right.is_none() {
                return if val < limit { None } else { root };
            }

            if left.is_some() {
                left = Self::traversal(left, limit - val);
            }
            if right.is_some() {
                right = Self::traversal(right, limit - val);
            }

            if left.is_none() && right.is_none() {
                None
            } else {
                r.borrow_mut().left = left;
                r.borrow_mut().right = right;
                root
            }
        } else {
            None
        }
    }

    pub fn sufficient_subset(
        root: Option<Rc<RefCell<TreeNode>>>,
        limit: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::traversal(root, limit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::sufficient_subset(
                TreeNode::from_vec(&[1, 2, 3, 4, -99, -99, 7, 8, 9, -99, -99, 12, 13, -99, 14]),
                1
            ),
            TreeNode::from_vec_opt(&[
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                None,
                None,
                Some(7),
                Some(8),
                Some(9),
                None,
                Some(14),
            ])
        );
        assert_eq!(
            Solution::sufficient_subset(
                TreeNode::from_vec_opt(&[
                    Some(1),
                    Some(2),
                    Some(-3),
                    Some(-5),
                    None,
                    Some(4),
                    None,
                ]),
                -1
            ),
            TreeNode::from_vec_opt(&[Some(1), None, Some(-3), Some(4),])
        );
    }
}
