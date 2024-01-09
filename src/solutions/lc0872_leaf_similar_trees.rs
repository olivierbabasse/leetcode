//! <https://leetcode.com/problems/leaf-similar-trees/>

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
        if root.left.is_none() && root.right.is_none() {
            output.push(root.val);
        }
        Self::traversal(&root.right, output);
    }

    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut output1 = Vec::new();
        Self::traversal(&root1, &mut output1);
        let mut output2 = Vec::new();
        Self::traversal(&root2, &mut output2);
        output1 == output2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert!(Solution::leaf_similar(
            TreeNode::from_vec_opt(&[
                Some(3),
                Some(5),
                Some(1),
                Some(6),
                Some(2),
                Some(9),
                Some(8),
                None,
                None,
                Some(7),
                Some(4)
            ]),
            TreeNode::from_vec_opt(&[
                Some(3),
                Some(5),
                Some(1),
                Some(6),
                Some(7),
                Some(4),
                Some(2),
                None,
                None,
                None,
                None,
                None,
                None,
                Some(9),
                Some(8)
            ])
        ));
        assert!(!Solution::leaf_similar(
            TreeNode::from_vec(&[1, 2, 3]),
            TreeNode::from_vec(&[1, 3, 2])
        ));
    }
}
