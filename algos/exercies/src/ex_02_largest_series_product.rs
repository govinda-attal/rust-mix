use std::collections::VecDeque;

pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

#[allow(dead_code)]
pub fn lsp(str_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        return Ok(1);
    }

    let result = str_digits
        .chars()
        .map(|c| c.to_digit(10).ok_or(Error::InvalidDigit(c)))
        .collect::<Result<Vec<u32>, Error>>()?;

    result
        .windows(span)
        .map(|w| w.iter().map(|&x| (x as u64)).product())
        .max()
        .ok_or(Error::SpanTooLong)
}

fn lsp_2(str_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        return Ok(1);
    }
    if span > str_digits.len() {
        return Err(Error::SpanTooLong);
    }

    let mut dq = VecDeque::new();
    let mut prod: u64 = 1;
    let mut max: u64 = 0;
    let mut zeros = 0;

    for c in str_digits.chars() {
        let n = c.to_digit(10);
        if n.is_none() {
            return Err(Error::InvalidDigit(c));
        }
        let n = n.unwrap();
        dq.push_front(n);
        if n == 0 {
            zeros += 1;
        } else {
            prod *= n as u64;
        }
        if dq.len() > span {
            let prev = dq.pop_back().unwrap();
            if prev == 0 {
                zeros -= 1;
            } else {
                prod = prod / prev as u64;
            }
        }
        if dq.len() == span && prod > max && zeros == 0 {
            max = prod;
        }
    }
    Ok(max)
}
