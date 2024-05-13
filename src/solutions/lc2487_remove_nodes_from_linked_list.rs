//! <https://leetcode.com/problems/remove-nodes-from-linked-list/>

type ListNode = crate::utils::list::ListNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    fn rec(node: Option<&mut Box<ListNode>>) -> i32 {
        if let Some(node) = node {
            if Self::rec(node.next.as_mut()) > node.val {
                *node = node.next.take().unwrap();
            }
            node.val
        } else {
            0
        }
    }

    pub fn remove_nodes(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::rec(head.as_mut());
        head
    }
}

#[cfg(test)]
mod tests {
    use super::{ListNode, Solution};

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::remove_nodes(ListNode::from_vec(&[5, 2, 13, 3, 8])),
            ListNode::from_vec(&[13, 8])
        );
        assert_eq!(
            Solution::remove_nodes(ListNode::from_vec(&[1, 1, 1, 1])),
            ListNode::from_vec(&[1, 1, 1, 1])
        );
    }
}
