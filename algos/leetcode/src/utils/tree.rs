use std::fmt::{Display, Formatter};

use std::cell::RefCell;
use std::rc::Rc;

pub type Tree = Option<Rc<RefCell<TreeNode>>>;
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Tree,
    pub right: Tree,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    pub fn from_vec(vals: &[i32]) -> Tree {
        if vals.is_empty() {
            return None;
        }

        let mut root = Self::new(vals[0]);
        root.fill(vals, 0);

        Some(Rc::new(RefCell::new(root)))
    }

    fn fill(&mut self, vals: &[i32], index: usize) {
        let left_node = index * 2 + 1;
        let right_node = left_node + 1;

        if left_node < vals.len() && vals[left_node] != i32::MIN {
            self.left = Some(Rc::new(RefCell::new(Self::new(vals[left_node]))));

            self.left
                .as_ref()
                .unwrap()
                .borrow_mut()
                .fill(vals, left_node);
        }

        if right_node < vals.len() && vals[right_node] != i32::MIN {
            self.right = Some(Rc::new(RefCell::new(Self::new(vals[right_node]))));

            self.right
                .as_ref()
                .unwrap()
                .borrow_mut()
                .fill(vals, right_node);
        }
    }

    pub fn to_vec(&self) -> Vec<i32> {
        let mut vals = Vec::new();
        self.walk(&mut vals);
        vals
    }

    fn walk(&self, result: &mut Vec<i32>) {
        if let Some(left) = &self.left {
            left.borrow().walk(result);
        }
        result.push(self.val);
        if let Some(right) = &self.right {
            right.borrow().walk(result);
        }
    }
}

impl Display for TreeNode {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        if self.val == i32::MIN {
            Ok(())
        } else {
            let left = if let Some(left) = &self.left {
                left.borrow().fmt(f)?;
                format!("{}", left.borrow().val)
            } else {
                "-".to_string()
            };
            let right = if let Some(right) = &self.right {
                right.borrow().fmt(f)?;
                format!("{}", right.borrow().val)
            } else {
                "-".to_string()
            };
            writeln!(f, "{}, left = {left}, right = {right}", self.val)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_tree() {
        assert!(TreeNode::from_vec(&[]).is_none());
    }

    #[test]
    fn fuller_tree() {
        let tree = TreeNode::from_vec(&[4, 2, 7, 1, 3, 6, 9]);
        let result = tree.unwrap().borrow().to_vec();
        assert_eq!(result, vec![1, 2, 3, 4, 6, 7, 9]);
    }

    #[test]
    fn null_entry_tree() {
        let tree = TreeNode::from_vec(&[6, 2, 8, 0, 4, 7, 9, i32::MIN, i32::MIN, 3, 5]);
        let result = tree.unwrap().borrow().to_vec();
        assert_eq!(result, vec![0, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
