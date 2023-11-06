//! <https://leetcode.com/problems/validate-binary-search-tree/>

use std::cell::RefCell;
use std::rc::Rc;

type TreeNode = crate::utils::tree::TreeNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    fn validate(root: &Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        if root.is_none() {
            return true;
        }

        let root = root.as_ref().unwrap().borrow();

        (root.val as i64) > min
            && (root.val as i64) < max
            && Self::validate(&root.left, min, root.val as i64)
            && Self::validate(&root.right, root.val as i64, max)
    }

    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::validate(&root, i64::MIN, i64::MAX)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert!(Solution::is_valid_bst(TreeNode::from_vec_opt(&[
            Some(2),
            Some(1),
            Some(3),
        ])),);
        assert!(!Solution::is_valid_bst(TreeNode::from_vec_opt(&[
            Some(5),
            Some(1),
            Some(4),
            None,
            None,
            Some(3),
            Some(6)
        ])),);
    }
}
