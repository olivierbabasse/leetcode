//! <https://leetcode.com/problems/find-bottom-left-tree-value/>

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type TreeNode = crate::utils::tree::TreeNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::new();
        queue.push_back(root);
        let mut res = 0;
        while !queue.is_empty() {
            let count = queue.len();
            for i in 0..count {
                if let Some(node) = queue.pop_front().unwrap() {
                    if i == 0 {
                        res = node.borrow().val;
                    }
                    if node.borrow().left.is_some() {
                        queue.push_back(node.borrow().left.clone());
                    }
                    if node.borrow().right.is_some() {
                        queue.push_back(node.borrow().right.clone());
                    }
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::find_bottom_left_value(TreeNode::from_vec(&[2, 1, 3])),
            1
        );
        assert_eq!(
            Solution::find_bottom_left_value(TreeNode::from_vec_opt(&[
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                None,
                Some(5),
                Some(6),
                None,
                None,
                Some(7)
            ])),
            7
        );
    }
}
