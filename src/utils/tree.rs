use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(PartialEq, Eq, Debug)]
pub struct TreeNode<T> {
    pub val: T,
    pub left: Option<Rc<RefCell<TreeNode<T>>>>,
    pub right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T: Copy> TreeNode<T> {
    pub fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    fn insert_bfs(vals: &[T], index: usize) -> Option<TreeNode<T>> {
        if index < vals.len() {
            let mut node = Self::new(vals[index]);
            if let Some(left) = Self::insert_bfs(vals, 2 * index + 1) {
                node.left = Some(Rc::new(RefCell::new(left)));
            }
            if let Some(right) = Self::insert_bfs(vals, 2 * index + 2) {
                node.right = Some(Rc::new(RefCell::new(right)));
            }
            Some(node)
        } else {
            None
        }
    }

    pub fn from_vec(vals: &[T]) -> Self {
        assert!(!vals.is_empty(), "array must not be empty");

        Self::insert_bfs(vals, 0).unwrap()
    }

    fn insert_bfs_opt(vals: &[Option<T>]) -> Option<TreeNode<T>> {
        let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
        let mut queue = VecDeque::new();
        queue.push_back(Rc::clone(&root));

        let mut index = 1;
        while index < vals.len() {
            if let Some(node) = queue.pop_front() {
                if let Some(val) = vals[index] {
                    let left = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().left = Some(Rc::clone(&left));
                    queue.push_back(Rc::clone(&left));
                }
                index += 1;

                if index < vals.len() && vals[index].is_some() {
                    let right = Rc::new(RefCell::new(TreeNode::new(vals[index].unwrap())));
                    node.borrow_mut().right = Some(Rc::clone(&right));
                    queue.push_back(Rc::clone(&right));
                }
                index += 1;
            }
        }

        Rc::try_unwrap(root).map(|root| root.into_inner()).ok()
    }

    pub fn from_vec_opt(vals: &[Option<T>]) -> Self {
        assert!(!vals.is_empty(), "array must not be empty");

        Self::insert_bfs_opt(vals).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_new() {
        assert_eq!(
            TreeNode::new(42),
            TreeNode {
                val: 42,
                left: None,
                right: None
            }
        );
    }

    #[test]
    #[should_panic]
    fn test_from_empty_vec() {
        TreeNode::<i32>::from_vec(&[]);
    }

    #[test]
    fn test_from_vec() {
        assert_eq!(
            TreeNode::from_vec(&[1, 2, 3]),
            TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None
                })))
            }
        );
    }

    #[test]
    fn test_from_vec_opt() {
        assert_eq!(
            TreeNode::from_vec_opt(&[Some(1), None, Some(3)]),
            TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None
                })))
            }
        );
        assert_eq!(
            TreeNode::from_vec_opt(&[Some(1), None, Some(2), Some(3)]),
            TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: None
                    }))),
                    right: None
                })))
            }
        );
    }
}
