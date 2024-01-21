//! <https://leetcode.com/problems/construct-binary-search-tree-from-preorder-traversal/>

use std::cell::RefCell;
use std::rc::Rc;

type TreeNode = crate::utils::tree::TreeNode<i32>;

struct Solution {}

impl Solution {
    fn rec(preorder: &[i32], left: usize, right: usize) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = TreeNode::new(preorder[left]);
        let mut i = right;
        while preorder[i] > preorder[left] {
            i -= 1;
        }

        if i > left {
            root.left = Self::rec(preorder, left + 1, i);
        }

        if i < right {
            root.right = Self::rec(preorder, i + 1, right);
        }

        Some(Rc::new(RefCell::new(root)))
    }

    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::rec(&preorder, 0, preorder.len() - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::bst_from_preorder(vec![8, 5, 1, 7, 10, 12]),
            TreeNode::from_vec_opt(&[Some(8), Some(5), Some(10), Some(1), Some(7), None, Some(12)])
        );
    }
}
