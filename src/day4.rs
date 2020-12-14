use nom::branch::alt;
use nom::bytes::complete::{tag, is_not, take_while_m_n};
use nom::character::complete::{digit1, multispace0};
use nom::combinator::{map_res, recognize};
use nom::IResult;
use nom::multi::{many0};
use nom::sequence::{pair, separated_pair, delimited};
use std::ops::Not;
use itertools::Itertools;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum HeightUnit {
    CM,
    IN,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Height {
    unit: HeightUnit,
    size: usize,
}

impl Height {
    fn from(size: usize, unit: HeightUnit) -> Option<Height> {
        let is_cm_valid = |n: usize| n >= 150 && n <= 193;
        let is_in_valid = |n: usize| n >= 59 && n <= 76;
        let valid = match unit {
            HeightUnit::CM => is_cm_valid(size),
            HeightUnit::IN => is_in_valid(size)
        };

        if valid {
            Some(Height { unit, size })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub enum PassportField {
    Byr(usize),
    Iyr(usize),
    Eyr(usize),
    Hgt(Height),
    Hcl(String),
    Ecl(String),
    Pid(usize),
    Cid(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Passport {
    byr: usize,
    iyr: usize,
    eyr: usize,
    hgt: Height,
    hcl: String,
    ecl: String,
    pid: usize,
    cid: Option<String>,
}

impl Passport {
    fn all_tags_required(tags: &[(String, String)]) -> bool {
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .all(|&tag| tags.iter().any(|(t, _)| t == tag))
    }

    fn from(tags: &[(String, String)]) -> Option<Passport> {
        if Passport::all_tags_required(tags).not() {
            None
        } else {
            let passport = Passport {
                byr: 0,
                iyr: 0,
                eyr: 0,
                hgt: Height { unit: HeightUnit::CM, size: 0 },
                hcl: "".to_string(),
                ecl: "".to_string(),
                pid: 0,
                cid: None
            };

            let tags = parse_tags(tags);
            let flat = tags.iter().flatten().collect_vec();
            if tags.len() != flat.len() {
                None
            } else {
                Some(flat
                    .iter()
                    .fold(passport, |acc, &tag| acc.set_field_from(tag)))
            }
        }
    }

    fn set_field_from(self, tag: &PassportField) -> Passport {
        match tag {
            PassportField::Byr(n) => Passport { byr: *n, ..self },
            PassportField::Iyr(n) => Passport { iyr: *n, ..self },
            PassportField::Eyr(n) => Passport { eyr: *n, ..self },
            PassportField::Hgt(h) => Passport { hgt: *h, ..self },
            PassportField::Hcl(s) => Passport { hcl: s.clone(), ..self },
            PassportField::Ecl(s) => Passport { ecl: s.clone(), ..self },
            PassportField::Pid(n) => Passport { pid: *n, ..self },
            PassportField::Cid(s) => Passport { cid: Some(s.clone()), ..self },
        }
    }
}

#[aoc_generator(day4)]
pub fn gen(input: &str) -> Vec<Vec<(String, String)>> {
    input
        .split("\n\n")
        .flat_map(|s| to_tag_value(s))
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(tags: &[Vec<(String, String)>]) -> usize {
    tags
        .iter()
        .fold(0, |acc, v| acc + if Passport::all_tags_required(v) { 1 } else { 0 })
}

#[aoc(day4, part2)]
pub fn solve_part2(tags: &[Vec<(String, String)>]) -> usize {
    tags
        .iter()
        .flat_map(|p| Passport::from(p))
        .count()
}

fn parse_hgt(s: &str) -> Option<PassportField> {
    fn take_number(s: &str) -> IResult<&str, usize> {
        map_res(digit1, |s: &str| s.parse())(s)
    }
    let hgt_in = pair(take_number, tag("in"));
    let hgt_cm = pair(take_number, tag("cm"));
    let alt_hgt: IResult<&str, _> = alt((hgt_in, hgt_cm))(s);

    match alt_hgt {
        Ok(("", (n, "in"))) => Some(PassportField::Hgt(Height::from(n, HeightUnit::IN)?)),
        Ok(("", (n, "cm"))) => Some(PassportField::Hgt(Height::from(n, HeightUnit::CM)?)),
        _ => None
    }
}

fn parse_number_if(s: &str, f: fn(&usize) -> bool) -> Option<usize> {
    s.parse()
        .ok()
        .filter(f)
}

fn parse_hcl(s: &str) -> Option<String> {
    let is_hexdigit = |c: char| c.is_digit(16);
    let take_n_hexdigit = |n| take_while_m_n(n, n, is_hexdigit);
    let ret: IResult<&str, _> = recognize(pair(tag("#"), take_n_hexdigit(6)))(s);

    match ret {
        Ok(("", x)) if s.len() == 7 => Some(x.to_string()),
        _ => None
    }
}

fn parse_tag((tag, value): (&str, &str)) -> Option<PassportField> {
    let is_byr_valid = |n: &usize| *n >= 1920 && *n <= 2002;
    let is_iyr_valid = |n: &usize| *n >= 2010 && *n <= 2020;
    let is_eyr_valid = |n: &usize| *n >= 2020 && *n <= 2030;
    let is_ecl_valid = |s: &str| {
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .iter()
            .any(|&col| s == col)
    };

    match tag {
        "byr" => Some(PassportField::Byr(parse_number_if(value, is_byr_valid)?)),
        "iyr" => Some(PassportField::Iyr(parse_number_if(value, is_iyr_valid)?)),
        "eyr" => Some(PassportField::Eyr(parse_number_if(value, is_eyr_valid)?)),
        "hgt" => parse_hgt(value),
        "hcl" => Some(PassportField::Hcl(parse_hcl(value)?)),
        "ecl" if is_ecl_valid(value) => Some(PassportField::Ecl(value.to_string())),
        "pid" if value.len() == 9 => Some(PassportField::Pid(parse_number_if(value, |_| true)?)),
        "cid" => Some(PassportField::Cid(value.to_string())),
        _ => None
    }
}

fn parse_tags(tags: &[(String, String)]) -> Vec<Option<PassportField>> {
    tags
        .iter()
        .map(|(tag, value)| parse_tag((&tag[..], &value[..])))
        .collect()
}

fn to_tag_value(s: &str) -> Option<Vec<(String, String)>> {
    let not_whitespace = is_not(" \t\n\r");
    let one_of_tags = alt((
        tag("byr"),
        tag("iyr"),
        tag("eyr"),
        tag("hgt"),
        tag("hcl"),
        tag("ecl"),
        tag("pid"),
        tag("cid")));
    let one_tag_separated = separated_pair(
        one_of_tags,
        tag(":"),
        not_whitespace);
    let one_tag_unspaced =
        delimited(
            multispace0,
            one_tag_separated,
            multispace0,
        );
    let all_tags: IResult<&str, _> = many0(one_tag_unspaced)(s);

    all_tags
        .map(|(_, tags)|
            tags.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
        )
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
        return "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
    }

    #[test]
    fn test_gen() {
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&gen(get_input())), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&gen(get_input())), 2);
    }
}