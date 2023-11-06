//! <https://leetcode.com/problems/find-mode-in-binary-search-tree/>

use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

type TreeNode = crate::utils::tree::TreeNode<i32>;

struct Solution {}

#[derive(Default)]
struct State {
    current_val: i32,
    current_count: usize,
    //    max_val: i32,
    max_count: usize,
}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    fn traversal(root: &Option<Rc<RefCell<TreeNode>>>, state: &mut State, output: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }

        let root = root.as_ref().unwrap().borrow();

        Self::traversal(&root.left, state, output);

        if root.val != state.current_val {
            state.current_val = root.val;
            state.current_count = 1;
        } else {
            state.current_count += 1;
        }

        match state.current_count.cmp(&state.max_count) {
            Ordering::Greater => {
                *output = vec![state.current_val];
                state.max_count = state.current_count;
            }
            Ordering::Equal => output.push(state.current_val),
            _ => {}
        }

        Self::traversal(&root.right, state, output);
    }

    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut state = State::default();
        let mut output = Vec::new();
        Self::traversal(&root, &mut state, &mut output);
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::find_mode(TreeNode::from_vec_opt(&[Some(1), None, Some(2), Some(2),])),
            vec![2]
        );
        assert_eq!(Solution::find_mode(TreeNode::from_vec(&[0])), vec![0]);
        assert_eq!(Solution::find_mode(TreeNode::from_vec(&[1, 1, 2])), vec![1]);
    }
}
