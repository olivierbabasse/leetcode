//! <https://leetcode.com/problems/cousins-in-binary-tree-ii/>

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type TreeNode = crate::utils::tree::TreeNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn replace_value_in_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        root.as_ref()?;

        let mut queue = VecDeque::new();
        queue.push_back(root.clone().unwrap());

        let mut sums = Vec::new();
        while !queue.is_empty() {
            let size = queue.len();
            let mut sum = 0;
            for _ in 0..size {
                let node = queue.pop_front().unwrap();
                let node = node.borrow();
                sum += node.val;
                if let Some(left) = node.left.as_ref() {
                    queue.push_back(left.clone());
                }
                if let Some(right) = node.right.as_ref() {
                    queue.push_back(right.clone());
                }
            }
            sums.push(sum);
        }

        let mut level = 1;
        root.as_ref().unwrap().borrow_mut().val = 0;
        queue.push_back(root.clone().unwrap());
        while !queue.is_empty() {
            let size = queue.len();
            for _ in 0..size {
                let node = queue.pop_front().unwrap();
                let node = node.borrow();
                let sum = node
                    .left
                    .as_ref()
                    .map(|n| n.borrow().val)
                    .unwrap_or_default()
                    + node
                        .right
                        .as_ref()
                        .map(|n| n.borrow().val)
                        .unwrap_or_default();
                if let Some(left) = node.left.as_ref() {
                    left.borrow_mut().val = sums[level] - sum;
                    queue.push_back(left.clone());
                }
                if let Some(right) = node.right.as_ref() {
                    right.borrow_mut().val = sums[level] - sum;
                    queue.push_back(right.clone());
                }
            }
            level += 1;
        }

        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::replace_value_in_tree(TreeNode::from_vec_opt(&[
                Some(5),
                Some(4),
                Some(9),
                Some(1),
                Some(10),
                None,
                Some(7)
            ])),
            TreeNode::from_vec_opt(&[Some(0), Some(0), Some(0), Some(7), Some(7), None, Some(11)])
        );
    }
}
