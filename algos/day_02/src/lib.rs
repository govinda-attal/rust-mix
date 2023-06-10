use std::{cmp::Ordering, str::FromStr};

#[derive(Debug, Eq, Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err(format!("not a known move {}!", s).to_string()),
        }
    }
}

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == &Move::Scissors && other == &Move::Rock {
            Some(Ordering::Less)
        } else if self == &Move::Rock && other == &Move::Scissors {
            Some(Ordering::Greater)
        } else {
            Some((*self as u8).cmp(&(*other as u8)))
        }
    }
}

pub fn process_part1(input: &str) -> String {
    let result: u32 = input
        .lines()
        .map(|line| {
            let moves = line
                .trim()
                .split(" ")
                .map(|s| s.parse::<Move>().unwrap())
                .collect::<Vec<Move>>();
            match moves[0].partial_cmp(&moves[1]) {
                Some(Ordering::Equal) => 3 + moves[1] as u32,
                Some(Ordering::Less) => 6 + moves[1] as u32,
                Some(Ordering::Greater) => 0 + moves[1] as u32,
                None => {
                    panic!("moves must be comparable");
                }
            }
        })
        .sum();
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let result: u32 = input
        .lines()
        .map(|line| {
            let moves = line
                .trim()
                .split(" ")
                .collect::<Vec<&str>>();
            let opponent_move = moves[0].parse::<Move>().unwrap();
            match moves[1] {
                "X" => {
                    let our_move = match opponent_move {
                        Move::Rock => Move::Scissors,
                        Move::Paper => Move::Rock,
                        Move::Scissors => Move::Paper,
                    };
                    our_move as u32
                },
                "Y" => {opponent_move as u32 + 3},
                "Z" => {
                    let our_move = match opponent_move {
                        Move::Rock => Move::Paper,
                        Move::Paper => Move::Scissors,
                        Move::Scissors => Move::Rock,
                    };
                    our_move as u32 + 6
                },
                _ => {
                    panic!("moves must be comparable");
                }
            }
        })
        .sum();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
    B X
    C Z";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!("15", result)
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!("12", result)
    }
}
