//! <https://leetcode.com/problems/flip-equivalent-binary-trees/>

use std::cell::RefCell;
use std::rc::Rc;

type TreeNode = crate::utils::tree::TreeNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn flip_equiv(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if root1.is_none() && root2.is_none() {
            return true;
        }
        if root1.is_none() || root2.is_none() {
            return false;
        }
        if root1.as_ref().unwrap().borrow().val != root2.as_ref().unwrap().borrow().val {
            return false;
        }
        let swapped = Self::flip_equiv(
            root1.as_ref().unwrap().borrow().left.clone(),
            root2.as_ref().unwrap().borrow().right.clone(),
        ) && Self::flip_equiv(
            root1.as_ref().unwrap().borrow().right.clone(),
            root2.as_ref().unwrap().borrow().left.clone(),
        );
        let not_swapped = Self::flip_equiv(
            root1.as_ref().unwrap().borrow().left.clone(),
            root2.as_ref().unwrap().borrow().left.clone(),
        ) && Self::flip_equiv(
            root1.as_ref().unwrap().borrow().right.clone(),
            root2.as_ref().unwrap().borrow().right.clone(),
        );
        swapped || not_swapped
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert!(Solution::flip_equiv(
            TreeNode::from_vec_opt(&[
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                Some(6),
                None,
                None,
                None,
                Some(7),
                Some(8)
            ]),
            TreeNode::from_vec_opt(&[
                Some(1),
                Some(3),
                Some(2),
                None,
                Some(6),
                Some(4),
                Some(5),
                None,
                None,
                None,
                None,
                Some(8),
                Some(7)
            ])
        ));
    }
}
