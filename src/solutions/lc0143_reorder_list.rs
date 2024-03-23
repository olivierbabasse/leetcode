//! <https://leetcode.com/problems/reorder-list/>

use std::collections::VecDeque;

type ListNode = crate::utils::list::ListNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut cur = head.as_ref();
        let mut deque = VecDeque::new();
        while let Some(node) = cur {
            deque.push_back(node.val);
            cur = node.next.as_ref();
        }
        let mut i = 0;
        let mut cur = head.as_mut();
        while let Some(node) = cur {
            match i % 2 == 0 {
                true => node.val = deque.pop_front().unwrap(),
                false => node.val = deque.pop_back().unwrap(),
            };
            cur = node.next.as_mut();
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ListNode, Solution};

    #[test]
    fn test_cases() {
        let mut l = ListNode::from_vec(&[1, 2, 3, 4, 5]);
        Solution::reorder_list(&mut l);
        assert_eq!(l, ListNode::from_vec(&[1, 5, 2, 4, 3]));
    }
}
