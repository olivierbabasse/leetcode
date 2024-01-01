//! <https://leetcode.com/problems/deepest-leaves-sum/>

use std::cell::RefCell;
use std::rc::Rc;

type TreeNode = crate::utils::tree::TreeNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    fn calc_depth(root: &Option<Rc<RefCell<TreeNode>>>, depth: i32, max_depth: &mut i32) {
        if root.is_none() {
            return;
        }

        if depth > *max_depth {
            *max_depth = depth;
        }

        let root = root.as_ref().unwrap().borrow();

        Self::calc_depth(&root.left, depth + 1, max_depth);
        Self::calc_depth(&root.right, depth + 1, max_depth);
    }

    fn sum_deepest(
        root: &Option<Rc<RefCell<TreeNode>>>,
        depth: i32,
        max_depth: i32,
        sum: &mut i32,
    ) {
        if root.is_none() {
            return;
        }

        let root = root.as_ref().unwrap().borrow();

        Self::sum_deepest(&root.left, depth + 1, max_depth, sum);
        if depth == max_depth {
            *sum += root.val;
        }
        Self::sum_deepest(&root.right, depth + 1, max_depth, sum);
    }

    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut depth = 0;
        Self::calc_depth(&root, 0, &mut depth);
        dbg!(&depth);

        let mut sum = 0;
        Self::sum_deepest(&root, 0, depth, &mut sum);
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::deepest_leaves_sum(TreeNode::from_vec_opt(&[
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                None,
                Some(6),
                Some(7),
                None,
                None,
                None,
                None,
                Some(8)
            ])),
            15
        );
    }
}
