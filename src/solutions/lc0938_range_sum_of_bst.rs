//! <https://leetcode.com/problems/range-sum-of-bst/>

use std::cell::RefCell;
use std::rc::Rc;

type TreeNode = crate::utils::tree::TreeNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    fn traversal(root: &Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32, output: &mut i32) {
        if root.is_none() {
            return;
        }

        let root = root.as_ref().unwrap().borrow();

        Self::traversal(&root.left, low, high, output);
        if low <= root.val && root.val <= high {
            *output += root.val;
        }
        Self::traversal(&root.right, low, high, output);
    }

    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut output = 0;
        Self::traversal(&root, low, high, &mut output);
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::range_sum_bst(
                TreeNode::from_vec_opt(&[
                    Some(10),
                    Some(5),
                    Some(15),
                    Some(3),
                    Some(7),
                    None,
                    Some(18)
                ]),
                7,
                15
            ),
            32
        );
    }
}
