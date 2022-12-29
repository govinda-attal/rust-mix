use std::collections::HashMap;

struct Solution;

impl Solution {
    fn majority_element(items: Vec<i32>) -> i32 {
        let mut hm = HashMap::new();
        let mut majority = (0, 0);

        for item in items {
            *hm.entry(item).or_insert(0) += 1;
            if hm[&item] > majority.1 {
                majority = (item, hm[&item]);
            }
        }
        // let mut vec = hm.into_iter().collect::<Vec<(i32, i32)>>();
        // vec.sort_by(|a, b| b.1.cmp(&a.1));
        // vec[0].0

        majority.0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_a() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3)
    }

    #[test]
    fn test_b() {
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2)
    }
}
