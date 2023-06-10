use std::collections::HashMap;

pub fn two_sum(items: Vec<i32>, sum: i32) -> Vec<i32> {
    
    let mut map = HashMap::new();

    items.into_iter().enumerate().find_map(|(i, item)| -> Option<Vec<_>> {
        let diff = sum - item;

        let Some(j) = map.get(&item) else {
            map.insert(diff, i);
            return None;
        };

        Some(vec![*j as i32, i as i32])
    }).unwrap_or_default()

}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn ex1()  {
        let mut rs = two_sum(vec![2,7,11,15], 9);
        rs.sort();
        assert_eq!(rs, vec![0,1])
    }
}