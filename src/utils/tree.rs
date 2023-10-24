use std::cell::RefCell;
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

    fn insert_bfs_opt(vals: &[Option<T>], index: usize) -> Option<TreeNode<T>> {
        if index < vals.len() {
            if let Some(val) = vals[index] {
                let mut node = Self::new(val);
                if let Some(left) = Self::insert_bfs_opt(vals, 2 * index + 1) {
                    node.left = Some(Rc::new(RefCell::new(left)));
                }
                if let Some(right) = Self::insert_bfs_opt(vals, 2 * index + 2) {
                    node.right = Some(Rc::new(RefCell::new(right)));
                }
                return Some(node);
            }
        }
        None
    }

    pub fn from_vec_opt(vals: &[Option<T>]) -> Self {
        assert!(!vals.is_empty(), "array must not be empty");

        Self::insert_bfs_opt(vals, 0).unwrap()
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
    }
}
