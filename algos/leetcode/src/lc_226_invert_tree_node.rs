use crate::utils::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    invert(&root)
}

fn invert(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match node {
        Some(node) => {
            let left = invert(&node.borrow().right);
            let right = invert(&node.borrow().left);
            Some(Rc::new(RefCell::new(TreeNode {
                val: node.borrow().val,
                left,
                right,
            })))
        }
        None => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_invert() {
        let tree = TreeNode::from_vec(&vec![4, 2, 7, 1, 3, 6, 9]);
        let inverted_tree = invert_tree(tree).unwrap().borrow().to_vec();

        assert_eq!(inverted_tree, vec![9, 7, 6, 4, 3, 2, 1]);

        let tree = TreeNode::from_vec(&vec![2, 1, 3]);
        let inverted_tree = invert_tree(tree).unwrap().borrow().to_vec();

        assert_eq!(inverted_tree, vec![3, 2, 1]);
    }

    #[test]
    fn check_three() {
        let tree = TreeNode::from_vec(&vec![2, 1, 3]);
        let inverted_tree = invert_tree(tree).unwrap().borrow().to_vec();

        assert_eq!(inverted_tree, vec![3, 2, 1]);
    }

    #[test]
    fn check_none() {
        let tree = TreeNode::from_vec(&vec![]);
        let inverted_tree = invert_tree(tree);

        assert_eq!(inverted_tree, None);
    }
}
