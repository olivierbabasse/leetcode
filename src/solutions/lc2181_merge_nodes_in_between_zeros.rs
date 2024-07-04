//! <https://leetcode.com/problems/merge-nodes-in-between-zeros/>

type ListNode = crate::utils::list::ListNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(mut node) = head.unwrap().next {
            let mut sum = 0;
            while node.val != 0 {
                sum += node.val;
                node = node.next.unwrap();
            }
            Some(Box::new(ListNode {
                val: sum,
                next: Self::merge_nodes(Some(node)),
            }))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ListNode, Solution};

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::merge_nodes(ListNode::from_vec(&[0, 3, 1, 0, 4, 5, 2, 0])),
            ListNode::from_vec(&[4, 11])
        );
        assert_eq!(
            Solution::merge_nodes(ListNode::from_vec(&[0, 1, 0, 3, 0, 2, 2, 0])),
            ListNode::from_vec(&[1, 3, 4])
        );
    }
}
