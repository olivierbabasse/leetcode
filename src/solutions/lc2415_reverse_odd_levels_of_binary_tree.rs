//! <https://leetcode.com/problems/reverse-odd-levels-of-binary-tree/>

use std::cell::RefCell;
use std::rc::Rc;

type TreeNode = crate::utils::tree::TreeNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    fn rec(node1: Option<Rc<RefCell<TreeNode>>>, node2: Option<Rc<RefCell<TreeNode>>>, level: i32) {
        if node1.is_none() || node2.is_none() {
            return;
        }

        let n1 = node1.clone().unwrap();
        let n2 = node2.clone().unwrap();

        if level % 2 == 0 {
            let temp = n1.borrow().val;
            n1.borrow_mut().val = n2.borrow().val;
            n2.borrow_mut().val = temp;
        }

        Self::rec(
            n1.borrow().left.clone(),
            n2.borrow().right.clone(),
            level + 1,
        );
        Self::rec(
            n1.borrow().right.clone(),
            n2.borrow().left.clone(),
            level + 1,
        );
    }

    pub fn reverse_odd_levels(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::rec(
            root.clone().unwrap().borrow().left.clone(),
            root.clone().unwrap().borrow().right.clone(),
            0,
        );
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::reverse_odd_levels(TreeNode::from_vec(&[2, 3, 5, 8, 13, 21, 34])),
            TreeNode::from_vec(&[2, 5, 3, 8, 13, 21, 34])
        );
        assert_eq!(
            Solution::reverse_odd_levels(TreeNode::from_vec(&[
                0, 1, 2, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2
            ])),
            TreeNode::from_vec(&[0, 2, 1, 0, 0, 0, 0, 2, 2, 2, 2, 1, 1, 1, 1])
        );
    }
}
