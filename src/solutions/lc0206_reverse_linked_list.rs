//! <https://leetcode.com/problems/reverse-linked-list/>

type ListNode = crate::utils::list::ListNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cur = &head;
        let mut new_list = None;
        while let Some(ref node) = cur {
            let mut new_node = ListNode::new(node.val);
            new_node.next = new_list;
            new_list = Some(Box::new(new_node));
            cur = &node.next;
        }
        new_list
    }
}

#[cfg(test)]
mod tests {
    use super::{ListNode, Solution};

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::reverse_list(ListNode::from_vec(&[1, 2, 3, 4, 5])),
            ListNode::from_vec(&[5, 4, 3, 2, 1])
        );
        assert_eq!(
            Solution::reverse_list(ListNode::from_vec(&[])),
            ListNode::from_vec(&[])
        );
    }
}
