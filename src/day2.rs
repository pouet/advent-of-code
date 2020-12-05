use std::char::ParseCharError;
use std::convert::Infallible;
use std::num::ParseIntError;
use std::str::FromStr;
use regex::Regex;

pub struct PasswordError;

impl From<Infallible> for PasswordError {
    fn from(_: Infallible) -> Self {
        PasswordError
    }
}

impl From<ParseCharError> for PasswordError {
    fn from(_: ParseCharError) -> Self {
        PasswordError
    }
}

impl From<ParseIntError> for PasswordError {
    fn from(_: ParseIntError) -> Self {
        PasswordError
    }
}

#[derive(Debug, PartialEq)]
pub struct Password {
    range: (usize, usize),
    letter: char,
    password: String,
}

impl FromStr for Password {
    type Err = PasswordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Regex::new(r"([0-9]+)-([0-9]+) ([a-zA-Z]+): (.*)")
            .map_err(|_| PasswordError)
            .and_then(|re|
                re
                    .captures(s)
                    .ok_or(PasswordError)
                    .and_then(|cap| Ok(Password {
                        range: (cap[1].parse()?, cap[2].parse()?),
                        letter: cap[3].parse()?,
                        password: cap[4].parse()?,
                    }))
            )
    }
}

impl Password {
    fn is_valid1(&self) -> bool {
        let count = self
            .password
            .matches(self.letter)
            .count();

        count >= self.range.0 && count <= self.range.1
    }

    fn is_valid2(&self) -> bool {
        let l1 = self.password.chars().nth(self.range.0 - 1).unwrap() == self.letter;
        let l2 = self.password.chars().nth(self.range.1 - 1).unwrap() == self.letter;

        (l1 && !l2) || (!l1 && l2)
    }
}

#[aoc_generator(day2)]
pub fn gen(input: &str) -> Vec<Password> {
    input
        .lines()
        .filter_map(|s| s.trim().parse().ok())
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<Password>) -> usize {
    input
        .iter()
        .filter(|pass| pass.is_valid1())
        .count()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Vec<Password>) -> usize {
    input
        .iter()
        .filter(|pass| pass.is_valid2())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
        return "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
    }

    #[test]
    fn test_gen() {
        let res = vec![
            Password { range: (1, 3), letter: 'a', password: String::from("abcde") },
            Password { range: (1, 3), letter: 'b', password: String::from("cdefg") },
            Password { range: (2, 9), letter: 'c', password: String::from("ccccccccc") }
        ];
        assert_eq!(gen(get_input()), res);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&gen(get_input())), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&gen(get_input())), 1);
    }
}