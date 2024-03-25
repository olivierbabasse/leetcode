//! <https://leetcode.com/problems/palindrome-linked-list/description/>

type ListNode = crate::utils::list::ListNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let dummy = Box::new(ListNode { val: 0, next: head });
        let (mut fast, mut slow) = (&dummy, &dummy);
        loop {
            if fast.next.is_none() || fast.next.as_ref().unwrap().next.is_none() {
                break;
            }
            fast = fast.next.as_ref().unwrap().next.as_ref().unwrap();
            slow = slow.next.as_ref().unwrap();
        }
        let mut prev = None;
        let mut reverse_start = slow.next.as_ref().unwrap().clone();
        loop {
            let next = reverse_start.next;
            reverse_start.next = prev;
            prev = Some(reverse_start);
            if next.is_none() {
                break;
            }
            reverse_start = next.unwrap();
        }
        let mut second_half = prev.as_ref().unwrap();
        let mut first_half = dummy.next.as_ref().unwrap();
        loop {
            if first_half.val != second_half.val {
                return false;
            }
            if second_half.next.is_none() {
                break;
            }
            first_half = first_half.next.as_ref().unwrap();
            second_half = second_half.next.as_ref().unwrap();
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::{ListNode, Solution};

    #[test]
    fn test_cases() {
        assert!(Solution::is_palindrome(ListNode::from_vec(&[1, 2, 2, 1])));
        assert!(!Solution::is_palindrome(ListNode::from_vec(&[1, 2])));
    }
}
