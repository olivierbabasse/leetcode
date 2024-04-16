//! <https://leetcode.com/problems/sum-of-left-leaves/>

use std::cell::RefCell;
use std::rc::Rc;

type TreeNode = crate::utils::tree::TreeNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    fn rec(node: &Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
        if let Some(node) = node {
            let node = node.borrow();
            if is_left && node.left.is_none() && node.right.is_none() {
                node.val
            } else {
                Self::rec(&node.left, true) + Self::rec(&node.right, false)
            }
        } else {
            0
        }
    }

    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::rec(&root, false)
    }
}

#[cfg(test)]
mod tests {
    use super::{Solution, TreeNode};

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::sum_of_left_leaves(TreeNode::from_vec_opt(&[
                Some(3),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ])),
            24
        );
        assert_eq!(Solution::sum_of_left_leaves(TreeNode::from_vec(&[1])), 0);
    }
}
