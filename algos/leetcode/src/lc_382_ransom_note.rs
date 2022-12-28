pub struct Solution;

impl Solution {
    pub fn can_construct(ransome_note: &str, magzine: &str) -> bool {
        use std::collections::HashMap;
        let mut dic = HashMap::new();
        for ch in magzine.chars() {
            *dic.entry(ch).or_insert(0) += 1;
        }
        for ch in ransome_note.chars() {
            let e = dic.entry(ch).or_insert(0);
            if *e == 0 {
                return false;
            }
            *e -= 1;
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a() {
        let (ransome_note, magzine) = ("aa", "aab");
        assert!(Solution::can_construct(ransome_note, magzine))
    }
    #[test]
    fn test_b() {
        let (ransome_note, magzine) = ("a", "b");
        assert!(!Solution::can_construct(ransome_note, magzine))
    }
    #[test]
    fn test_c() {
        let (ransome_note, magzine) = ("ab", "aa");
        assert!(!Solution::can_construct(ransome_note, magzine))
    }
}