//! <https://leetcode.com/problems/add-two-numbers/>

type ListNode = crate::utils::list::ListNode<i32>;

struct Solution1 {}

/// naive implementation, iterative
/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution1 {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut acc = 0;
        let mut head = Box::new(ListNode::new(0));
        let mut tail = &mut head;
        while l1.is_some() || l2.is_some() || acc != 0 {
            let d1 = l1.as_ref().map(|node| node.val).unwrap_or(0);
            let d2 = l2.as_ref().map(|node| node.val).unwrap_or(0);
            let mut sum = d1 + d2 + acc;
            if sum >= 10 {
                sum -= 10;
                acc = 1;
            } else {
                acc = 0;
            }

            if let Some(l) = l1 {
                l1 = l.next;
            }
            if let Some(l) = l2 {
                l2 = l.next;
            }

            tail.next = Some(Box::new(ListNode::new(sum)));
            tail = tail.next.as_mut().unwrap();
        }
        head.next
    }
}

struct Solution2 {}

/// recursive
/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution2 {
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
    use super::{ListNode, Solution1, Solution2};

    #[test]
    fn test_cases_1() {
        assert_eq!(
            Solution1::add_two_numbers(
                Some(Box::new(ListNode::from_vec(&[2, 4, 3]))),
                Some(Box::new(ListNode::from_vec(&[5, 6, 4])))
            ),
            Some(Box::new(ListNode::from_vec(&[7, 0, 8])))
        );
        assert_eq!(
            Solution1::add_two_numbers(
                Some(Box::new(ListNode::from_vec(&[0]))),
                Some(Box::new(ListNode::from_vec(&[0])))
            ),
            Some(Box::new(ListNode::from_vec(&[0])))
        );
        assert_eq!(
            Solution1::add_two_numbers(
                Some(Box::new(ListNode::from_vec(&[9, 9, 9, 9, 9, 9, 9]))),
                Some(Box::new(ListNode::from_vec(&[9, 9, 9, 9])))
            ),
            Some(Box::new(ListNode::from_vec(&[8, 9, 9, 9, 0, 0, 0, 1])))
        );
    }

    #[test]
    fn test_cases_2() {
        assert_eq!(
            Solution2::add_two_numbers(
                Some(Box::new(ListNode::from_vec(&[2, 4, 3]))),
                Some(Box::new(ListNode::from_vec(&[5, 6, 4])))
            ),
            Some(Box::new(ListNode::from_vec(&[7, 0, 8])))
        );
        assert_eq!(
            Solution2::add_two_numbers(
                Some(Box::new(ListNode::from_vec(&[0]))),
                Some(Box::new(ListNode::from_vec(&[0])))
            ),
            Some(Box::new(ListNode::from_vec(&[0])))
        );
        assert_eq!(
            Solution2::add_two_numbers(
                Some(Box::new(ListNode::from_vec(&[9, 9, 9, 9, 9, 9, 9]))),
                Some(Box::new(ListNode::from_vec(&[9, 9, 9, 9])))
            ),
            Some(Box::new(ListNode::from_vec(&[8, 9, 9, 9, 0, 0, 0, 1])))
        );
    }
}
