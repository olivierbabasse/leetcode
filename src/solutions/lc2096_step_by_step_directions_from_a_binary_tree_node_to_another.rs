//! <https://leetcode.com/problems/step-by-step-directions-from-a-binary-tree-node-to-another/>

use std::cell::RefCell;
use std::rc::Rc;

type TreeNode = crate::utils::tree::TreeNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    fn find_path(root: &Option<Rc<RefCell<TreeNode>>>, target: i32, path: &mut String) -> bool {
        if let Some(root) = root {
            if root.borrow().val == target {
                return true;
            }

            path.push('L');
            if Self::find_path(&root.borrow().left, target, path) {
                return true;
            }
            path.pop();

            path.push('R');
            if Self::find_path(&root.borrow().right, target, path) {
                return true;
            }
            path.pop();
        }
        false
    }

    pub fn get_directions(
        root: Option<Rc<RefCell<TreeNode>>>,
        start_value: i32,
        dest_value: i32,
    ) -> String {
        let (mut start_path, mut dest_path) = (String::new(), String::new());
        Self::find_path(&root, start_value, &mut start_path);
        Self::find_path(&root, dest_value, &mut dest_path);

        let prefix_len = start_path
            .chars()
            .zip(dest_path.chars())
            .take_while(|(c1, c2)| c1 == c2)
            .count();

        let mut directions = "U".repeat(start_path.len() - prefix_len);
        directions.push_str(&dest_path[prefix_len..]);
        directions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::get_directions(
                TreeNode::from_vec_opt(&[
                    Some(5),
                    Some(1),
                    Some(2),
                    Some(3),
                    None,
                    Some(6),
                    Some(4)
                ]),
                3,
                6
            ),
            "UURL".to_string()
        );
        assert_eq!(
            Solution::get_directions(TreeNode::from_vec(&[2, 1]), 2, 1),
            "L".to_string()
        );
    }
}
