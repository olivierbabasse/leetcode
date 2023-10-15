#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ListNode<T> {
    pub val: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl<T: Copy> ListNode<T> {
    pub fn new(val: T) -> Self {
        Self { val, next: None }
    }

    pub fn from_vec(vals: &[T]) -> Self {
        assert!(!vals.is_empty(), "array must not be empty");

        let mut node = Self::new(vals[0]);
        node.next = if vals.len() > 1 {
            Some(Box::new(Self::from_vec(&vals[1..])))
        } else {
            None
        };
        node
    }
}

#[cfg(test)]
mod tests {
    use super::ListNode;

    #[test]
    fn test_new() {
        assert_eq!(
            ListNode::new(42),
            ListNode {
                val: 42,
                next: None
            }
        );
    }

    #[test]
    #[should_panic]
    fn test_from_empty_vec() {
        ListNode::<i32>::from_vec(&[]);
    }

    #[test]
    fn test_from_vec() {
        assert_eq!(
            ListNode::from_vec(&[1, 2]),
            ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 2, next: None }))
            }
        );
    }
}
