//! <https://leetcode.com/problems/add-two-numbers/>

type ListNode = crate::utils::list::ListNode<i32>;

struct Solution {}

/// recursive
/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (Some(node), None) | (None, Some(node)) => Some(node),
            (Some(node1), Some(node2)) => {
                let sum = node1.val + node2.val;
                if sum < 10 {
                    Some(Box::new(ListNode {
                        val: sum,
                        next: Self::add_two_numbers(node1.next, node2.next),
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: sum - 10,
                        next: Self::add_two_numbers(
                            Self::add_two_numbers(node1.next, Some(Box::new(ListNode::new(1)))),
                            node2.next,
                        ),
                    }))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ListNode, Solution};

    #[test]
    fn test_cases_2() {
        assert_eq!(
            Solution::add_two_numbers(
                ListNode::from_vec(&[2, 4, 3]),
                ListNode::from_vec(&[5, 6, 4])
            ),
            ListNode::from_vec(&[7, 0, 8])
        );
        assert_eq!(
            Solution::add_two_numbers(ListNode::from_vec(&[0]), ListNode::from_vec(&[0])),
            ListNode::from_vec(&[0])
        );
        assert_eq!(
            Solution::add_two_numbers(
                ListNode::from_vec(&[9, 9, 9, 9, 9, 9, 9]),
                ListNode::from_vec(&[9, 9, 9, 9])
            ),
            ListNode::from_vec(&[8, 9, 9, 9, 0, 0, 0, 1])
        );
    }
}
