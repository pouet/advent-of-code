use nom::IResult;
use nom::sequence::{delimited, pair, tuple};
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, multispace0, digit1};
use nom::multi::{many1, many0};
use nom::combinator::{map_res, opt, recognize};
use itertools::Itertools;
use nom::branch::alt;
use nom::lib::std::collections::HashMap;

#[derive(Debug)]
pub enum Expr {
    Letter(char),
    Numbers(Vec<Vec<usize>>),
}

fn parse_number(s: &str) -> IResult<&str, usize> {
    let (s, n) = map_res(digit1, |s: &str| s.parse::<usize>())(s)?;
    Ok((s, n))
}

fn parse_number_list(s: &str) -> IResult<&str, Vec<usize>> {
    let (s, d) = many1(pair(multispace0, parse_number))(s)?;
    let d = d
        .into_iter()
        .map(|(_, num)| num)
        .collect();
    Ok((s, d))
}

fn parse_or(s: &str) -> IResult<&str, ()> {
    let (s, _) = recognize(tuple((multispace0, tag("|"), multispace0)))(s)?;
    Ok((s, ()))
}

fn parse_numbers(s: &str) -> IResult<&str, Expr> {
    let (s, nums) = parse_number_list(s)?;
    let (s, opts) = opt(many0(pair(parse_or, parse_number_list)))(s)?;

    let mut opts = opts
        .into_iter()
        .flatten()
        .map(|(_, nums)| nums)
        .collect_vec();

    let mut v = vec![nums];
    v.append(&mut opts);

    Ok((s, Expr::Numbers(v)))
}

fn parse_letter(s: &str) -> IResult<&str, Expr> {
    let (s, _) = multispace0(s)?;
    let (s, c) = delimited(tag("\""), anychar, tag("\""))(s)?;
    Ok((s, Expr::Letter(c)))
}

fn parse_line(s: &str) -> IResult<&str, (usize, Expr)> {
    let (s, n) = parse_number(s)?;
    let (s, _) = tag(":")(s)?;
    let (s, expr) = alt((parse_numbers, parse_letter))(s)?;
    Ok((s, (n, expr)))
}

fn parse(s: &str) -> HashMap<usize, Expr> {
    s
        .lines()
        .flat_map(|line| parse_line(line))
        .map(|(_, e)| e)
        .collect()
}

fn validate_one<'a>(h: &HashMap<usize, Expr>, s: &'a str, expr: &Vec<usize>) -> Option<&'a str> {
    if s.is_empty() {
        return None;
    }

    let mut s = s;
    for num in expr {
        s = validate(h, s, &h[num])?;
    }

    Some(s)
}

fn validate<'a>(h: &HashMap<usize, Expr>, s: &'a str, expr: &Expr) -> Option<&'a str> {
    match expr {
        Expr::Letter(c) if s.starts_with(*c) => Some(&s[1..]),
        Expr::Numbers(nums) => nums
            .iter()
            .find_map(|expr| validate_one(h, s, expr)),
        _ => None
    }
}

fn validate_msg(h: &HashMap<usize, Expr>, s: &str) -> bool {
    match validate(h, s, &h[&0]) {
        Some(s) if s.is_empty() => true,
        _ => false
    }
}

#[aoc_generator(day19)]
pub fn gen(input: &str) -> (HashMap<usize, Expr>, Vec<String>) {
    let mut sp = input.split("\n\n");
    let h = parse(sp.next().unwrap());
    let msgs = sp.next().unwrap().lines().map(|s| s.to_string()).collect();

    (h, msgs)
}

#[aoc(day19, part1)]
pub fn solve_part1((h, msgs): &(HashMap<usize, Expr>, Vec<String>)) -> usize {
    msgs
        .iter()
        .filter(|s| validate_msg(h, &s[..]))
        .count()
}

fn part2(h: &HashMap<usize, Expr>, s: &str) -> bool {
    let mut s = s;
    let nums = vec![42, 31];
    let mut ret = vec![0, 0];

    for (i, n) in nums.iter().enumerate() {
        while let Some(t) = validate(h, s, &h[n]) {
            ret[i] += 1;
            s = t;
        }
    }

    s.is_empty() && ret[1] > 0 && ret[0] > ret[1]
}

#[aoc(day19, part2)]
pub fn solve_part2((h, msgs): &(HashMap<usize, Expr>, Vec<String>)) -> usize {
    msgs
        .iter()
        .filter(|s| part2(h, &s[..]))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
        return "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";
    }

    fn get_input2() -> &'static str {
        return "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";
    }


    #[test]
    fn test_gen() {
        // println!("{:?}", gen(get_input()));
        // let h = parse(get_input());
        // println!("validate: {:?}", validate(&h, "ababbb", &h[&0]));
        // println!("validate: {:?}", validate(&h, "bababa", &h[&0]));
        // println!("validate: {:?}", validate(&h, "abbbab", &h[&0]));
        // println!("validate: {:?}", validate(&h, "aaabbb", &h[&0]));
        // println!("validate: {:?}", validate(&h, "aaaabbb", &h[&0]));
        //
        // println!("validate: {:?}", validate_msg(&h, "ababbb"));
        // println!("validate: {:?}", validate_msg(&h, "bababa"));
        // println!("validate: {:?}", validate_msg(&h, "abbbab"));
        // println!("validate: {:?}", validate_msg(&h, "aaabbb"));
        // println!("validate: {:?}", validate_msg(&h, "aaaabbb"));
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&gen(get_input())), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&gen(get_input2())), 12);
    }
}