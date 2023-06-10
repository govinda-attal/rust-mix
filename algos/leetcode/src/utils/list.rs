#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }

    pub fn from_vec(vals: &[i32]) -> Option<Box<ListNode>> {
        let mut result = None;
        for entry in vals.iter().rev() {
            let mut node = Self::new(*entry);
            node.next = result;
            result = Some(Box::new(node));
        }
        result
    }

    pub fn to_vec(&self) -> Vec<i32> {
        let mut result = vec![self.val];

        let mut head = &self.next;
        while let Some(node) = head {
            result.push(node.val);
            head = &node.next;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_list() {
        let mut list = ListNode::from_vec(&vec![1, 2, 3, 4, 5]);

        let mut n = 1;
        while let Some(entry) = list {
            assert_eq!(entry.val, n);
            n += 1;
            list = entry.next
        }
    }

    #[test]
    fn back_to_vec() {
        let src_vec = vec![1, 2, 3, 4, 5];

        let conv_vec = ListNode::from_vec(&src_vec).unwrap().to_vec();

        assert_eq!(conv_vec, src_vec)
    }
}
