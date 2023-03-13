fn is_palindrome(num: i32) -> bool {
    let mut rev = 0;
    let mut x = num;

    while x > 0 {
        rev <<= 1;
        rev |= x & 1;
        x >>= 1;
    }

    rev == num
}

#[cfg(test)]
mod tests {
    use crate::ex_06_palindrome::is_palindrome;

    #[test]
    fn test_palindrome() {
        println!("5 as bits: {:#b}", 5);
        println!("3 as bits: {:#b}", 3);
        println!("4 as bits: {:#b}", 4);
        assert!(is_palindrome(5));
        assert!(is_palindrome(3));
        assert!(!is_palindrome(4));
    }
}
