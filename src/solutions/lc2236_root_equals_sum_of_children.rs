//! <https://leetcode.com/problems/root-equals-sum-of-children/>

use std::cell::RefCell;
use std::rc::Rc;

type TreeNode = crate::utils::tree::TreeNode<i32>;

struct Solution {}

/// time-complexity : O(1)
/// space-complexity : O(1)
impl Solution {
    pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let root = root.as_ref().unwrap().borrow();
        let left = root.left.as_ref().unwrap().borrow();
        let right = root.right.as_ref().unwrap().borrow();
        root.val == left.val + right.val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert!(Solution::check_tree(Some(Rc::new(RefCell::new(
            TreeNode {
                val: 10,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: None,
                    right: None
                })))
            }
        )))));
        assert!(!Solution::check_tree(Some(Rc::new(RefCell::new(
            TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None
                })))
            }
        )))));
    }
}
