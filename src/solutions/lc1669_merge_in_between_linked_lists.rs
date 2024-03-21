//! <https://leetcode.com/problems/merge-in-between-linked-lists/>

type ListNode = crate::utils::list::ListNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn merge_in_between(
        list1: Option<Box<ListNode>>,
        a: i32,
        b: i32,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut before_list = Box::new(ListNode::new(0));
        before_list.next = list1;
        let mut cut_start = &mut before_list;
        for _ in 0..a {
            cut_start = cut_start.next.as_mut().unwrap()
        }
        let mut cut_stop = &mut cut_start.next;
        for _ in a..=b {
            cut_stop = &mut cut_stop.as_mut().unwrap().next
        }
        let end = cut_stop.take();
        cut_start.next = list2;
        while let Some(ref mut next) = cut_start.next {
            cut_start = next;
        }
        cut_start.next = end;
        before_list.next
    }
}

#[cfg(test)]
mod tests {
    use super::{ListNode, Solution};

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::merge_in_between(
                ListNode::from_vec(&[10, 1, 13, 6, 9, 5]),
                3,
                4,
                ListNode::from_vec(&[1000000, 1000001, 1000002])
            ),
            ListNode::from_vec(&[10, 1, 13, 1000000, 1000001, 1000002, 5])
        );
    }
}
