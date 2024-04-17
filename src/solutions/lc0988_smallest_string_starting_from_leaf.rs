//! <https://leetcode.com/problems/smallest-string-starting-from-leaf/>

use std::cell::RefCell;
use std::rc::Rc;

type TreeNode = crate::utils::tree::TreeNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(treeheight)
impl Solution {
    fn rec(node: &Option<Rc<RefCell<TreeNode>>>, cur: &mut String, best: &mut String) {
        if let Some(node) = node {
            let node = node.borrow();
            cur.push((node.val + 'a' as i32) as u8 as char);

            if node.left.is_none() && node.right.is_none() {
                let rcur = cur.chars().rev().collect::<String>();
                if best.is_empty() || rcur < *best {
                    *best = rcur;
                }
            }

            Solution::rec(&node.left, cur, best);
            Solution::rec(&node.right, cur, best);

            cur.pop();
        }
    }

    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut res = String::new();
        Self::rec(&root, &mut String::new(), &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::{Solution, TreeNode};

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::smallest_from_leaf(TreeNode::from_vec(&[0, 1, 2, 3, 4, 3, 4])),
            "dba".to_string()
        );
        assert_eq!(
            Solution::smallest_from_leaf(TreeNode::from_vec(&[25, 1, 3, 1, 3, 0, 2])),
            "adz".to_string()
        );
    }
}
