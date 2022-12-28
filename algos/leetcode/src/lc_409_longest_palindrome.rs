struct Solution;

impl Solution {
    pub fn longest_palindrome(s: &str) -> i32 {
        use std::collections::HashMap;

        let mut hm = HashMap::new();
        for ch  in s.chars() {
            *hm.entry(ch).or_insert(0) += 1;
        }

        let mut max_len = 0;
        let mut has_odd = false;
        for val in hm.values() {
            if *val % 2 == 0 {
                max_len += val
            } else {
                max_len += val - 1;
                has_odd = true;
            }
        }

        max_len + if has_odd {1} else {0}
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_a() {
        let s = "abccccdd";
        assert_eq!(Solution::longest_palindrome(s), 7)
    }
    #[test]
    fn test_b() {
        let s = "a";
        assert_eq!(Solution::longest_palindrome(s), 1)
    }

    #[test]
    fn test_c() {
        let s = "aaab";
        assert_eq!(Solution::longest_palindrome(s), 3)
    }
}