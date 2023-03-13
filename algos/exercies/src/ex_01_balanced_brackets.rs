#[allow(dead_code)]

fn balanced_brackets(input: &str) -> bool {
    let mut stack = vec![];
    const BRACES: [char; 3] = ['{', '[', '('];
    input.chars().all(|c| match c {
        '}' => stack.pop() == Some('{'),
        ')' => stack.pop() == Some('('),
        ']' => stack.pop() == Some('['),
        _ => true,
    }) && stack.len() == 0
}
