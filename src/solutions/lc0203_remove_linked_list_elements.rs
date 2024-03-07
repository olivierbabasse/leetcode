//! <https://leetcode.com/problems/remove-linked-list-elements/>

type ListNode = crate::utils::list::ListNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut list = None;
        let mut tail = &mut list;

        while let Some(mut node) = head.take() {
            head = node.next.take();
            if node.val != val {
                tail = &mut tail.insert(node).next;
            }
        }

        list
    }
}

#[cfg(test)]
mod tests {
    use super::{ListNode, Solution};

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::remove_elements(ListNode::from_vec(&[1, 2, 6, 3, 4, 5, 6]), 6),
            ListNode::from_vec(&[1, 2, 3, 4, 5])
        );
        assert_eq!(
            Solution::remove_elements(ListNode::from_vec(&[]), 1),
            ListNode::from_vec(&[])
        );
        assert_eq!(
            Solution::remove_elements(ListNode::from_vec(&[7, 7, 7, 7]), 7),
            ListNode::from_vec(&[])
        );
    }
}
