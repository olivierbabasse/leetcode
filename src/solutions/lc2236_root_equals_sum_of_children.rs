//! <https://leetcode.com/problems/root-equals-sum-of-children/>

use std::cell::RefCell;
use std::rc::Rc;

type TreeNode = crate::utils::tree::TreeNode<i32>;

struct Solution {}

/// time-complexity : O(1)
/// space-complexity : O(1)
impl Solution {
    pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let root = root.as_ref().unwrap().borrow();
        let left = root.left.as_ref().unwrap().borrow();
        let right = root.right.as_ref().unwrap().borrow();
        root.val == left.val + right.val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert!(Solution::check_tree(TreeNode::from_vec(&[10, 4, 6])));
        assert!(!Solution::check_tree(TreeNode::from_vec(&[5, 3, 1])));
    }
}
