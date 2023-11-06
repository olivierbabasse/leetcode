//! <https://leetcode.com/problems/find-largest-value-in-each-tree-row/>

use std::cell::RefCell;
use std::rc::Rc;

type TreeNode = crate::utils::tree::TreeNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, depth: usize, maximums: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }

        let root = root.as_ref().unwrap().borrow();

        if maximums.len() <= depth {
            maximums.push(root.val);
        } else {
            maximums[depth] = maximums[depth].max(root.val);
        }

        Self::dfs(&root.left, depth + 1, maximums);
        Self::dfs(&root.right, depth + 1, maximums);
    }

    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut maximums = Vec::new();
        Self::dfs(&root, 0, &mut maximums);
        maximums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::largest_values(TreeNode::from_vec_opt(&[
                Some(1),
                Some(3),
                Some(2),
                Some(5),
                Some(3),
                None,
                Some(9)
            ])),
            vec![1, 3, 9]
        );
        assert_eq!(
            Solution::largest_values(TreeNode::from_vec(&[1, 2, 3])),
            vec![1, 3]
        );
    }
}
