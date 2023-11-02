//! <https://leetcode.com/problems/count-nodes-equal-to-average-of-subtree/>

use std::cell::RefCell;
use std::rc::Rc;

type TreeNode = crate::utils::tree::TreeNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    fn subtree_sum(root: &Option<Rc<RefCell<TreeNode>>>, output: &mut i32) -> (i32, usize) {
        if let Some(root) = root {
            let root = root.borrow();

            let (left_sum, left_count) = Self::subtree_sum(&root.left, output);
            let (right_sum, right_count) = Self::subtree_sum(&root.right, output);

            let (sum, count) = (
                left_sum + right_sum + root.val,
                left_count + right_count + 1,
            );

            if sum / (count as i32) == root.val {
                *output += 1;
            }

            (sum, count)
        } else {
            (0, 0)
        }
    }

    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut output = 0;
        Self::subtree_sum(&root, &mut output);
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::average_of_subtree(Some(Rc::new(RefCell::new(TreeNode::from_vec_opt(&[
                Some(4),
                Some(8),
                Some(5),
                Some(0),
                Some(1),
                None,
                Some(6),
            ]))))),
            5
        );
        assert_eq!(
            Solution::average_of_subtree(Some(Rc::new(RefCell::new(TreeNode::from_vec(&[1]))))),
            1
        );
    }
}
