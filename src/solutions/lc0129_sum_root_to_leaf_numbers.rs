//! <https://leetcode.com/problems/sum-root-to-leaf-numbers/>

use std::cell::RefCell;
use std::rc::Rc;

type TreeNode = crate::utils::tree::TreeNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    fn rec(node: Option<Rc<RefCell<TreeNode>>>, cur: i32, total: &mut i32) {
        if let Some(node) = node {
            let node = node.borrow();
            if node.left.is_some() || node.right.is_some() {
                Self::rec(node.left.clone(), cur * 10 + node.val, total);
                Self::rec(node.right.clone(), cur * 10 + node.val, total);
            } else {
                *total += cur * 10 + node.val;
            }
        }
    }

    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut total = 0;
        Self::rec(root, 0, &mut total);
        total
    }
}

#[cfg(test)]
mod tests {
    use super::{Solution, TreeNode};

    #[test]
    fn test_cases() {
        assert_eq!(Solution::sum_numbers(TreeNode::from_vec(&[1, 2, 3])), 25);
        assert_eq!(
            Solution::sum_numbers(TreeNode::from_vec(&[4, 9, 0, 5, 1])),
            1026
        );
    }
}
