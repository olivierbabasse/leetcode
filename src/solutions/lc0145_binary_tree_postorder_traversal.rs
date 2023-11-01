//! <https://leetcode.com/problems/binary-tree-postorder-traversal/>

use std::cell::RefCell;
use std::rc::Rc;

type TreeNode = crate::utils::tree::TreeNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    fn traversal(root: &Option<Rc<RefCell<TreeNode>>>, output: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }

        let root = root.as_ref().unwrap().borrow();

        Self::traversal(&root.left, output);
        Self::traversal(&root.right, output);
        output.push(root.val);
    }

    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut output = Vec::new();
        Self::traversal(&root, &mut output);
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::postorder_traversal(Some(Rc::new(RefCell::new(TreeNode::from_vec_opt(&[
                Some(1),
                None,
                Some(2),
                Some(3),
            ]))))),
            vec![3, 2, 1]
        );
        assert_eq!(Solution::postorder_traversal(None), vec![]);
        assert_eq!(
            Solution::postorder_traversal(Some(Rc::new(RefCell::new(TreeNode::from_vec(&[1]))))),
            vec![1]
        );
    }
}
