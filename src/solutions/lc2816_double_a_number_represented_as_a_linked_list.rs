//! <https://leetcode.com/problems/double-a-number-represented-as-a-linked-list/>

type ListNode = crate::utils::list::ListNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    fn rec(node: Option<&mut Box<ListNode>>) -> i32 {
        if let Some(node) = node {
            node.val = node.val * 2 + Self::rec(node.next.as_mut());
            if node.val >= 10 {
                node.val -= 10;
                return 1;
            }
        }
        0
    }

    pub fn double_it(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if Self::rec(head.as_mut()) > 0 {
            Some(Box::new(ListNode { val: 1, next: head }))
        } else {
            head
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ListNode, Solution};

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::double_it(ListNode::from_vec(&[1, 8, 9])),
            ListNode::from_vec(&[3, 7, 8])
        );
        assert_eq!(
            Solution::double_it(ListNode::from_vec(&[9, 9, 9])),
            ListNode::from_vec(&[1, 9, 9, 8])
        );
    }
}
