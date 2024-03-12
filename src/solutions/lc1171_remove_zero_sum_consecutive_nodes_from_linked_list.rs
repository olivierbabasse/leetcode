//! <https://leetcode.com/problems/remove-zero-sum-consecutive-nodes-from-linked-list/>

use std::collections::HashMap;

type ListNode = crate::utils::list::ListNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut psum_node = HashMap::new();
        let mut new_list = Box::new(ListNode { val: 0, next: head });

        let mut psum = 0;
        let mut index = 0;
        let mut current = &new_list;
        loop {
            psum += current.val;
            psum_node.insert(psum, index);
            if current.next.is_none() {
                break;
            }
            current = &current.next.as_ref().unwrap();
            index += 1;
        }

        psum = 0;
        index = 0;
        let mut current = &mut new_list;
        loop {
            psum += current.val;
            if let Some(&end) = psum_node.get(&psum) {
                while index < end {
                    current.next = current.next.as_mut().unwrap().next.take();
                    index += 1;
                }
            }
            if current.next.is_none() {
                break;
            }
            current = current.next.as_mut().unwrap();
            index += 1;
        }

        new_list.next
    }
}

#[cfg(test)]
mod tests {
    use super::{ListNode, Solution};

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::remove_zero_sum_sublists(ListNode::from_vec(&[1, 2, -3, 3, 1])),
            ListNode::from_vec(&[3, 1])
        );
        assert_eq!(
            Solution::remove_zero_sum_sublists(ListNode::from_vec(&[1, 2, 3, -3, 4])),
            ListNode::from_vec(&[1, 2, 4])
        );
        assert_eq!(
            Solution::remove_zero_sum_sublists(ListNode::from_vec(&[1, 2, 3, -3, -2])),
            ListNode::from_vec(&[1])
        );
    }
}
