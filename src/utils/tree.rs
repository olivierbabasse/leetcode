use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Eq, Debug)]
pub struct TreeNode<T> {
    pub val: T,
    pub left: Option<Rc<RefCell<TreeNode<T>>>>,
    pub right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> TreeNode<T> {
    pub fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TreeNode;

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
}
