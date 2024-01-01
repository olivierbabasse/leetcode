//! <https://leetcode.com/problems/insert-greatest-common-divisors-in-linked-list/>

type ListNode = crate::utils::list::ListNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    fn gcd(mut n: i32, mut m: i32) -> i32 {
        while m != 0 {
            if m < n {
                std::mem::swap(&mut m, &mut n);
            }
            m %= n;
        }
        n
    }

    pub fn insert_greatest_common_divisors(
        mut head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut cur = &mut head;
        while cur.is_some() && cur.as_ref().unwrap().next.is_some() {
            let mut newnode = ListNode::new(Self::gcd(
                cur.as_ref().unwrap().val,
                cur.as_ref().unwrap().next.as_ref().unwrap().val,
            ));
            std::mem::swap(&mut cur.as_mut().unwrap().next, &mut newnode.next);
            cur.as_mut().unwrap().next = Some(Box::new(newnode));
            cur = &mut cur.as_mut().unwrap().next.as_mut().unwrap().next;
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::insert_greatest_common_divisors(ListNode::from_vec(&[18, 6, 10, 3])),
            ListNode::from_vec(&[18, 6, 6, 2, 10, 1, 3])
        );
    }
}
