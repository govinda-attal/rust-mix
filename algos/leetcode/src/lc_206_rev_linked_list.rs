use crate::utils::ListNode;

struct Solution;
impl Solution {
    pub fn reverse_list(mut root: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut items = vec![];
        let mut ptr = None;

        while let Some(node) = root {
            items.push(node.val);
            root = node.next;
        }

        for n in items {
            let mut node = ListNode::new(n);
            node.next = ptr;
            ptr = Some(Box::new(node));
        }
        ptr
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reverse() {
        let list = ListNode::from_vec(&vec![1,2,3,4,5]);
        let rev_list = Solution::reverse_list(list).unwrap();

        let rev_vec = rev_list.to_vec();
        assert_eq!(vec![5,4,3,2,1], rev_vec)
    }
    #[test]
    fn test_reverse_2_elems() {
        let list = ListNode::from_vec(&vec![1,2]);
        let rev_list = Solution::reverse_list(list).unwrap();

        let rev_vec = rev_list.to_vec();
        assert_eq!(vec![2,1], rev_vec)
    }
    #[test]
    fn test_reverse_no_elems() {

        assert!(Solution::reverse_list(ListNode::from_vec(&vec![])).is_none())
    }
}