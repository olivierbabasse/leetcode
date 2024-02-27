//! <https://leetcode.com/problems/diameter-of-binary-tree/>

use std::cell::RefCell;
use std::rc::Rc;

type TreeNode = crate::utils::tree::TreeNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(log(n))
impl Solution {
    fn rec(tree: &Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
        if tree.is_none() {
            return 0;
        }

        let left = Self::rec(&tree.as_ref().unwrap().borrow().left, max);
        let right = Self::rec(&tree.as_ref().unwrap().borrow().right, max);
        *max = (*max).max(left + right);
        1 + left.max(right)
    }

    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = 0;
        Self::rec(&root, &mut max);
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::diameter_of_binary_tree(TreeNode::from_vec(&[1, 2, 3, 4, 5])),
            3
        );
        assert_eq!(
            Solution::diameter_of_binary_tree(TreeNode::from_vec(&[1, 2])),
            1
        );
    }
}
