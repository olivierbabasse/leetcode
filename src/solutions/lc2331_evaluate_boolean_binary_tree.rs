//! <https://leetcode.com/problems/evaluate-boolean-binary-tree/>

use std::cell::RefCell;
use std::rc::Rc;

type TreeNode = crate::utils::tree::TreeNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(log(n))
impl Solution {
    fn eval(root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        let root = root.as_ref().unwrap().borrow();
        match root.val {
            0 => false,
            1 => true,
            2 => Self::eval(&root.left) || Self::eval(&root.right),
            3 => Self::eval(&root.left) && Self::eval(&root.right),
            _ => unreachable!(),
        }
    }

    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::eval(&root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert!(Solution::evaluate_tree(TreeNode::from_vec_opt(&[
            Some(2),
            Some(1),
            Some(3),
            None,
            None,
            Some(0),
            Some(1)
        ])));
    }
}
