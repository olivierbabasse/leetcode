//! <https://leetcode.com/problems/middle-of-the-linked-list/>

type ListNode = crate::utils::list::ListNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = &head;
        let mut fast = &head;

        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }

        slow.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::{ListNode, Solution};

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::middle_node(Some(Box::new(ListNode::from_vec(&[1, 2, 3, 4, 5]))),),
            Some(Box::new(ListNode::from_vec(&[3, 4, 5])))
        );
        assert_eq!(
            Solution::middle_node(Some(Box::new(ListNode::from_vec(&[1, 2, 3, 4, 5, 6]))),),
            Some(Box::new(ListNode::from_vec(&[4, 5, 6])))
        );
    }
}
