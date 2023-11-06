//! <https://leetcode.com/problems/merge-two-sorted-lists/>

type ListNode = crate::utils::list::ListNode<i32>;

struct Solution {}

/// time-complexity : O(m+n)
/// space-complexity : O(m+n)
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(l1), Some(l2)) => {
                if l1.val < l2.val {
                    Some(Box::new(ListNode {
                        val: l1.val,
                        next: Self::merge_two_lists(l1.next, Some(l2)),
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: l2.val,
                        next: Self::merge_two_lists(Some(l1), l2.next),
                    }))
                }
            }
            (Some(l1), None) => Some(l1),
            (None, Some(l2)) => Some(l2),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::merge_two_lists(
                ListNode::from_vec(&[1, 2, 4]),
                ListNode::from_vec(&[1, 3, 4])
            ),
            ListNode::from_vec(&[1, 1, 2, 3, 4, 4])
        );
        assert_eq!(
            Solution::merge_two_lists(ListNode::from_vec(&[]), ListNode::from_vec(&[])),
            ListNode::from_vec(&[])
        );
        assert_eq!(
            Solution::merge_two_lists(ListNode::from_vec(&[]), ListNode::from_vec(&[0])),
            ListNode::from_vec(&[0])
        );
    }
}
