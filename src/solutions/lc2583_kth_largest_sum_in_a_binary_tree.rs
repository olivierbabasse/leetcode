//! <https://leetcode.com/problems/kth-largest-sum-in-a-binary-tree/>

use std::cell::RefCell;
use std::collections::{BinaryHeap, VecDeque};
use std::rc::Rc;

type TreeNode = crate::utils::tree::TreeNode<i32>;

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        if root.is_none() {
            return -1;
        }
        let mut queue = VecDeque::new();
        queue.push_back(root.unwrap());

        let mut max_heap = BinaryHeap::new();
        while !queue.is_empty() {
            let size = queue.len();
            let mut sum = 0i64;
            for _ in 0..size {
                let node = queue.pop_front().unwrap();
                let node = node.borrow();
                sum += node.val as i64;
                if let Some(left) = node.left.as_ref() {
                    queue.push_back(left.clone());
                }
                if let Some(right) = node.right.as_ref() {
                    queue.push_back(right.clone());
                }
            }
            max_heap.push(sum);
        }

        for _ in 0..k as usize - 1 {
            max_heap.pop();
        }
        max_heap.pop().unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::kth_largest_level_sum(TreeNode::from_vec(&[5, 8, 9, 2, 1, 3, 7, 4, 6]), 2),
            13
        );
        assert_eq!(
            Solution::kth_largest_level_sum(
                TreeNode::from_vec_opt(&[Some(1), Some(2), None, Some(3)]),
                1
            ),
            3
        );
    }
}
