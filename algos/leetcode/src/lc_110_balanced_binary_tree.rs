use std::{cell::RefCell, rc::Rc};

use crate::utils::{Tree, TreeNode};
pub struct Solution;

impl Solution {
    pub fn is_balanced(&self, root: Tree) -> bool {
        Self::dfs(&root).is_some()
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
        match root {
            None => Some(0),
            Some(root) => {
                let root_ref = root.borrow();
                let lh = Self::dfs(&root_ref.left);
                let rh = Self::dfs(&root_ref.right);
                match (lh, rh) {
                    (Some(lh), Some(rh)) => {
                        if (lh - rh).abs() < 2 {
                            Some(lh.max(rh) + 1)
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }
        }
    }
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_balanced_one() {
        let tree = TreeNode::from_vec(&vec![3, 9, 20, i32::MIN, i32::MIN, i32::MIN, 15, 7]);
        let sol = Solution::new();
        assert!(sol.is_balanced(tree))
    }

    #[test]
    fn test_un_balanced_one() {
        let tree = TreeNode::from_vec(&vec![1, 2, 2, 3,3, i32::MIN, i32::MIN, i32::MIN, 4, 4]);
        let sol = Solution::new();
        assert!(!sol.is_balanced(tree))
    }
}
