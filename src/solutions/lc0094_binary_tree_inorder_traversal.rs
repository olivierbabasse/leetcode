//! <https://leetcode.com/problems/binary-tree-inorder-traversal/>

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
        output.push(root.val);
        Self::traversal(&root.right, output);
    }

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
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
            Solution::inorder_traversal(TreeNode::from_vec_opt(
                &[Some(1), None, Some(2), Some(3),]
            )),
            vec![1, 3, 2]
        );
        assert_eq!(Solution::inorder_traversal(None), vec![]);
        assert_eq!(
            Solution::inorder_traversal(TreeNode::from_vec(&[1])),
            vec![1]
        );
    }
}
