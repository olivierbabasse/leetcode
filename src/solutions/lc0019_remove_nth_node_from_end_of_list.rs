//! <https://leetcode.com/problems/remove-nth-node-from-end-of-list/>

type ListNode = crate::utils::list::ListNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut first = head.clone();
        let mut second = &mut head;
        for _ in 0..n {
            first = first.unwrap().next;
        }
        while let Some(node) = first {
            first = node.next;
            second = &mut second.as_mut().unwrap().next;
        }
        *second = second.as_mut().map(|node| node.next.clone()).unwrap();
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::remove_nth_from_end(ListNode::from_vec(&[1, 2, 3, 4, 5]), 2),
            ListNode::from_vec(&[1, 2, 3, 5])
        );
        assert_eq!(
            Solution::remove_nth_from_end(ListNode::from_vec(&[1]), 1),
            ListNode::from_vec(&[])
        );
        assert_eq!(
            Solution::remove_nth_from_end(ListNode::from_vec(&[1, 2]), 1),
            ListNode::from_vec(&[1])
        );
    }
}
