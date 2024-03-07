//! <https://leetcode.com/problems/delete-the-middle-node-of-a-linked-list/>

type ListNode = crate::utils::list::ListNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = &head.clone();
        let mut slow = &mut head;

        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &mut slow.as_mut().unwrap().next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }

        *slow = (*slow).as_mut().unwrap().next.take();

        head
    }
}

#[cfg(test)]
mod tests {
    use super::{ListNode, Solution};

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::delete_middle(ListNode::from_vec(&[1, 3, 4, 7, 1, 2, 6])),
            ListNode::from_vec(&[1, 3, 4, 1, 2, 6])
        );
        assert_eq!(
            Solution::delete_middle(ListNode::from_vec(&[1, 2, 3, 4])),
            ListNode::from_vec(&[1, 2, 4])
        );
        assert_eq!(
            Solution::delete_middle(ListNode::from_vec(&[2, 1])),
            ListNode::from_vec(&[2])
        );
    }
}
