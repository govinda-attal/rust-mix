use std::cmp::Ordering;

fn find<T: Ord, V: AsRef<[T]>>(nums: V, val: T) -> Option<usize> {
    let nums = nums.as_ref();
    if nums.len() == 0 {
        return None;
    }
    let (mut l, mut r) = (0 as usize, nums.len());

    while l <= r {
        let mid = (l + r) / 2;
        match nums[mid].cmp(&val) {
            Ordering::Equal => return Some(mid),
            Ordering::Greater => r = mid,
            Ordering::Less => l = mid + 1,
        }
        // if val == nums[mid] {
        //   return Some(mid);
        // }
        // if val > nums[mid] {
        //   l = mid + 1
        // } else if val < nums[mid] {
        //   r = mid
        // }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::ex_04_binary_search::*;

    #[test]
    fn find_simple() {
        let nums = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(find(nums, 3), Some(3))
    }
}
