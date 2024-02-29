//! <https://leetcode.com/problems/even-odd-tree/>

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type TreeNode = crate::utils::tree::TreeNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut queue = VecDeque::new();
        queue.push_back(root);
        let mut even_level = true;
        let mut last = 0;
        while !queue.is_empty() {
            let count = queue.len();
            for i in 0..count {
                if let Some(node) = queue.pop_front().unwrap() {
                    let val = node.borrow().val;
                    if (even_level && val % 2 == 0) || (!even_level && val % 2 != 0) {
                        return false;
                    }
                    if i != 0 && ((even_level && val <= last) || (!even_level && val >= last)) {
                        return false;
                    }
                    last = val;
                    if node.borrow().left.is_some() {
                        queue.push_back(node.borrow().left.clone());
                    }
                    if node.borrow().right.is_some() {
                        queue.push_back(node.borrow().right.clone());
                    }
                }
            }
            even_level = !even_level;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert!(Solution::is_even_odd_tree(TreeNode::from_vec_opt(&[
            Some(1),
            Some(10),
            Some(4),
            Some(3),
            None,
            Some(7),
            Some(9),
            Some(12),
            Some(8),
            Some(6),
            None,
            None,
            Some(2)
        ])));
        assert!(!Solution::is_even_odd_tree(TreeNode::from_vec(&[
            5, 4, 2, 3, 3, 7
        ])));
        assert!(!Solution::is_even_odd_tree(TreeNode::from_vec(&[
            5, 9, 1, 3, 5, 7
        ])));
    }
}
