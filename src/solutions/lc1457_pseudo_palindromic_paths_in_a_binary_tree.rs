//! <https://leetcode.com/problems/pseudo-palindromic-paths-in-a-binary-tree/>

use std::cell::RefCell;
use std::rc::Rc;

type TreeNode = crate::utils::tree::TreeNode<i32>;

struct Solution {}

/// time-complexity : O()
/// space-complexity : O()
impl Solution {
    fn traversal(root: &Option<Rc<RefCell<TreeNode>>>, freqs: &mut [usize; 10], total: &mut usize) {
        if root.is_none() {
            return;
        }

        let root = root.as_ref().unwrap().borrow();

        freqs[root.val as usize] += 1;

        if root.left.is_none() && root.right.is_none() {
            if freqs.iter().filter(|&f| f % 2 != 0).count() <= 1 {
                *total += 1;
            }
        } else {
            Self::traversal(&root.left, freqs, total);
            Self::traversal(&root.right, freqs, total);
        }

        freqs[root.val as usize] -= 1;
    }

    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut freqs = [0; 10];
        let mut total = 0;
        Self::traversal(&root, &mut freqs, &mut total);
        total as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::pseudo_palindromic_paths(TreeNode::from_vec_opt(&[
                Some(2),
                Some(3),
                Some(1),
                Some(3),
                Some(1),
                None,
                Some(1)
            ])),
            2
        );
        assert_eq!(
            Solution::pseudo_palindromic_paths(TreeNode::from_vec_opt(&[
                Some(2),
                Some(1),
                Some(1),
                Some(1),
                Some(3),
                None,
                None,
                None,
                None,
                None,
                Some(1)
            ])),
            1
        );
        assert_eq!(
            Solution::pseudo_palindromic_paths(TreeNode::from_vec(&[9])),
            1
        );
    }
}
