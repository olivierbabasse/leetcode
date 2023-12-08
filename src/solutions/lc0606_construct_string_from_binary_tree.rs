//! <https://leetcode.com/problems/construct-string-from-binary-tree/>

use std::cell::RefCell;
use std::rc::Rc;

type TreeNode = crate::utils::tree::TreeNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn rec(root: &Option<Rc<RefCell<TreeNode>>>) -> String {
        if root.is_none() {
            return String::new();
        }

        let root = root.as_ref().unwrap().borrow();
        let mut output = format!("{}", root.val);

        let left = Self::rec(&root.left);
        if !left.is_empty() {
            output.push_str(&format!("({})", left));
        }

        let right = Self::rec(&root.right);
        if !right.is_empty() {
            if left.is_empty() {
                output.push_str("()");
            }
            output.push_str(&format!("({})", right));
        }

        output
    }

    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        Self::rec(&root)
    }
}

#[cfg(test)]
mod tests {
    use super::{Solution, TreeNode};

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::tree2str(TreeNode::from_vec(&[1, 2, 3, 4])),
            "1(2(4))(3)".to_string()
        );
        assert_eq!(
            Solution::tree2str(TreeNode::from_vec_opt(&[
                Some(1),
                Some(2),
                Some(3),
                None,
                Some(4)
            ])),
            "1(2()(4))(3)".to_string()
        );
    }
}
