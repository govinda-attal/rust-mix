pub fn is_valid(input: &str) -> bool {
    // let mut stack = String::new();

    // for ch in input.chars() {
    //     match ch {
    //         '{' => stack.push('}'),
    //         '(' => stack.push(')'),
    //         '[' => stack.push(']'),
    //         '}' | ')' | ']' => {
    //             let popped = stack.pop().unwrap_or_default();

    //             if popped != ch {
    //                 return false;
    //             }
    //         }
    //         _ => return false,
    //     }
    // }
    // true


    let mut stack = String::new();

    for x in input.chars() {
        let ok = match x {
            ']' => stack.pop().unwrap_or_default() == '[',
            '}' => stack.pop().unwrap_or_default() == '{',
            ')' => stack.pop().unwrap_or_default() == '(',
            '[' => { stack.push(x); continue;},
            '(' => { stack.push(x); continue;},
            '{' => { stack.push(x); continue;},
            _ => false,
        };
        if !ok {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_valid() {
        let input = r"()[]{}";
        assert!(is_valid(input))
    }

    #[test]
    fn check_empty() {
        let input = r"";
        assert!(is_valid(input))
    }

    #[test]
    fn check_invalid() {
        let input = r"{{))";
        assert!(!is_valid(input))
    }
}
