use crate::utils::ListNode;

pub fn merge_sorted_list(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    if list1.is_none() {
        return list2;
    }

    if list2.is_none() {
        return list1;
    }

    let mut list1 = list1;
    let mut list2 = list2;
    let mut vec_result = Vec::new();

    while list1.is_some() && list2.is_some() {
        let v1 = list1.as_ref().unwrap().val;
        let v2 = list2.as_ref().unwrap().val;

        if v1 < v2 {
            vec_result.push(v1);
            list1 = list1.unwrap().next;
        } else {
            vec_result.push(v2);
            list2 = list2.unwrap().next;
        }
    }

    while let Some(entry) = list1 {
        vec_result.push(entry.val);
        list1 = entry.next;
    }

    while let Some(entry) = list2 {
        vec_result.push(entry.val);
        list2 = entry.next;
    }

    ListNode::from_vec(&vec_result)
}

#[cfg(test)]
mod test {
    use crate::utils::ListNode;

    use super::merge_sorted_list;

    #[test]
    fn check_simple() {
        let list1 = ListNode::from_vec(&vec![1, 2, 4]);
        let list2 = ListNode::from_vec(&vec![3, 5]);

        let result = merge_sorted_list(list1, list2);

        assert!(result.is_some());
        assert_eq!(result.unwrap().to_vec(), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn check_overlapping() {
        let list1 = ListNode::from_vec(&vec![1, 2, 4]);
        let list2 = ListNode::from_vec(&vec![1, 4]);

        let result = merge_sorted_list(list1, list2);

        assert!(result.is_some());
        assert_eq!(result.unwrap().to_vec(), vec![1, 1, 2, 4, 4]);
    }
}
