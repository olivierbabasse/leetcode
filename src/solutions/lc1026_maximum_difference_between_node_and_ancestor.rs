//! <https://leetcode.com/problems/maximum-difference-between-node-and-ancestor/>

use std::cell::RefCell;
use std::rc::Rc;

type TreeNode = crate::utils::tree::TreeNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    fn traversal(
        root: &Option<Rc<RefCell<TreeNode>>>,
        mut minval: Option<i32>,
        mut maxval: Option<i32>,
        maxdiff: &mut i32,
    ) {
        if root.is_none() {
            return;
        }

        let root = root.as_ref().unwrap().borrow();

        if minval.is_none() || maxval.is_none() {
            minval = Some(root.val);
            maxval = Some(root.val);
        }

        *maxdiff = i32::max(
            *maxdiff,
            i32::max(
                i32::abs(minval.unwrap() - root.val),
                i32::abs(maxval.unwrap() - root.val),
            ),
        );
        minval = minval.map(|v| i32::min(v, root.val));
        maxval = maxval.map(|v| i32::max(v, root.val));
        Self::traversal(&root.left, minval, maxval, maxdiff);
        Self::traversal(&root.right, minval, maxval, maxdiff);
    }

    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut maxdiff = 0;
        Self::traversal(&root, None, None, &mut maxdiff);
        maxdiff
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::max_ancestor_diff(TreeNode::from_vec_opt(&[
                Some(8),
                Some(3),
                Some(10),
                Some(1),
                Some(6),
                None,
                Some(14),
                None,
                None,
                Some(4),
                Some(7),
                Some(13)
            ])),
            7
        );
        assert_eq!(
            Solution::max_ancestor_diff(TreeNode::from_vec_opt(&[
                Some(1),
                None,
                Some(2),
                None,
                Some(0),
                Some(3)
            ])),
            3
        );
    }
}
