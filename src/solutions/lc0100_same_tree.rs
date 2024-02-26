//! <https://leetcode.com/problems/same-tree/>

use std::cell::RefCell;
use std::rc::Rc;

type TreeNode = crate::utils::tree::TreeNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn rec(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        if p.is_none() || q.is_none() {
            return p == q;
        }

        let p = p.as_ref().unwrap().borrow();
        let q = q.as_ref().unwrap().borrow();
        p.val == q.val && Self::rec(&p.left, &q.left) && Self::rec(&p.right, &q.right)
    }

    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Self::rec(&p, &q)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert!(Solution::is_same_tree(
            TreeNode::from_vec(&[1, 2, 3]),
            TreeNode::from_vec(&[1, 2, 3])
        ));
        assert!(!Solution::is_same_tree(
            TreeNode::from_vec(&[1, 2]),
            TreeNode::from_vec_opt(&[Some(1), None, Some(2)])
        ));
        assert!(!Solution::is_same_tree(
            TreeNode::from_vec(&[1, 2, 1]),
            TreeNode::from_vec(&[1, 1, 2])
        ));
    }
}
