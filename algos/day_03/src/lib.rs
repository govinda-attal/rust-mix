#![feature(iter_array_chunks)]
use std::collections::HashMap;

pub fn letter_scores_map() -> HashMap<char, usize> {
    ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>()
}

pub fn process_part1(input: &str) -> String {
    let letter_scores = letter_scores_map();

    let result: usize = input
        .lines()
        .map(|line| {
            let sack_len = line.len() / 2;
            let cmpt_a = &line[..sack_len];
            let cmpt_b = &line[sack_len..];

            let common = cmpt_a.chars().find(|c| cmpt_b.contains(*c)).unwrap();
            letter_scores.get(&common).unwrap()
        })
        .sum();
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let letter_scores = letter_scores_map();
    let result = input.lines().array_chunks::<3>()
    .map(|[a, b, c]| {
       let common_char = a.chars().find(|a_char|b.contains(*a_char) && c.contains(*a_char)).unwrap();
       letter_scores.get(&common_char).unwrap()
    }).sum::<usize>();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!("157", result)
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!("70", result)
    }
}
