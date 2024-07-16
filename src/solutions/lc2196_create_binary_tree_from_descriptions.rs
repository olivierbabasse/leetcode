//! <https://leetcode.com/problems/create-binary-tree-from-descriptions/>

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

type TreeNode = crate::utils::tree::TreeNode<i32>;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut nodes = HashSet::new();
        let mut children = HashSet::new();
        let mut edges = HashMap::<_, Vec<(i32, bool)>>::new();

        for d in descriptions {
            nodes.insert(d[0]);
            nodes.insert(d[1]);
            children.insert(d[1]);
            edges.entry(d[0]).or_default().push((d[1], d[2] == 1));
        }

        let root = Rc::new(RefCell::new(TreeNode::new(
            *nodes.difference(&children).next().unwrap(),
        )));

        let mut queue = Vec::new();
        queue.push(root.clone());
        while let Some(parent) = queue.pop() {
            let val = parent.borrow().val;
            if let Some(edges) = edges.get(&val) {
                for (val, left) in edges {
                    let newnode = Rc::new(RefCell::new(TreeNode::new(*val)));
                    queue.push(newnode.clone());
                    if *left {
                        parent.borrow_mut().left = Some(newnode);
                    } else {
                        parent.borrow_mut().right = Some(newnode);
                    }
                }
            }
        }

        Some(root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::create_binary_tree(vec![
                vec![20, 15, 1],
                vec![20, 17, 0],
                vec![50, 20, 1],
                vec![50, 80, 0],
                vec![80, 19, 1]
            ]),
            TreeNode::from_vec(&[50, 20, 80, 15, 17, 19])
        );
        assert_eq!(
            Solution::create_binary_tree(vec![vec![1, 2, 1], vec![2, 3, 0], vec![3, 4, 1]]),
            TreeNode::from_vec_opt(&[Some(1), Some(2), None, None, Some(3), Some(4)])
        );
    }
}
