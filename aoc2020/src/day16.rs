use nom::bytes::complete::{tag, take_while};
use nom::sequence::{tuple, separated_pair, pair};
use nom::character::complete::{space1, digit1, multispace0, char, newline};
use nom::combinator::{recognize, map_res, opt};
use nom::IResult;
use nom::multi::many1;
use itertools::Itertools;
use std::ops::Not;
use nom::lib::std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
pub struct Range {
    lower: usize,
    upper: usize,
}

#[derive(Debug, Clone)]
pub struct Field {
    name: String,
    ranges: [Range; 2],
}

#[derive(Debug, Clone)]
pub struct Ticket {
    values: Vec<usize>
}

#[derive(Debug, Clone)]
pub struct State {
    fields: Vec<Field>,
    ticket: Ticket,
    nearby: Vec<Ticket>,
}

impl Range {
    fn in_bounds(&self, value: &usize) -> bool {
        *value >= self.lower && *value <= self.upper
    }
}

impl Field {
    fn in_bounds(&self, value: &usize) -> bool {
        self.ranges.iter().any(|r| r.in_bounds(value))
    }
}

impl Ticket {
    fn is_valid(&self, fields: &[Field]) -> bool {
        self.values
            .iter()
            .all(|val| fields
                .iter()
                .any(|f| f.in_bounds(val))
            )
    }

    fn get_invalid_values(&self, fields: &[Field]) -> Vec<usize> {
        self.values
            .iter()
            .filter(|val| fields
                .iter()
                .all(|f| f.in_bounds(val).not())// {
            )
            .copied()
            .collect()
    }
}

impl State {
    fn get_columns(&self) -> Vec<Vec<usize>> {
        let mut ret: Vec<Vec<usize>> = Vec::new();

        for i in 0..self.ticket.values.len() {
            let mut v: Vec<usize> = Vec::new();
            for j in 0..self.nearby.len() {
                v.push(self.nearby[j].values[i]);
            }
            ret.push(v);
        }

        ret
    }
}

fn parse_num(s: &str) -> IResult<&str, usize> {
    let parse_num = |s: &str| s.parse::<usize>();
    map_res(digit1, parse_num)(s)
}

fn parse_fields(s: &str) -> IResult<&str, Vec<Field>> {
    fn field(s: &str) -> IResult<&str, String> {
        let not_separator = |c: char| c != ':';
        let (s, field) = take_while(not_separator)(s)?;
        Ok((&s[1..], field.to_string()))
    }

    fn range(s: &str) -> IResult<&str, Range> {
        let (s, (lower, upper)) = separated_pair(parse_num, tag("-"), parse_num)(s)?;
        Ok((s, Range { lower, upper }))
    }

    fn ranges(s: &str) -> IResult<&str, (Range, Range)> {
        let or = recognize(separated_pair(space1, tag("or"), space1));
        let (s, _) = multispace0(s)?;
        let (s, (n, m)) = separated_pair(range, or, range)(s)?;
        Ok((s, (n, m)))
    }

    let (s, fields) = many1(tuple((field, ranges, newline)))(s)?;
    let fields = fields
        .iter()
        .map(|(name, (r1, r2), _)|
            Field { name: name.clone(), ranges: [*r1, *r2] }
        ).collect_vec();

    Ok((s, fields))
}

fn parse_ticket(s: &str) -> IResult<&str, Ticket> {
    let (s, _) = multispace0(s)?;
    let (s, values) = many1(tuple((parse_num, opt(char(',')))))(s)?;
    let values = values
        .iter()
        .map(|(n, _)| *n)
        .collect();
    Ok((s, Ticket { values }))
}

fn parse_mine_ticket(s: &str) -> IResult<&str, Ticket> {
    let (s, _) = multispace0(s)?;
    let (s, _) = pair(tag("your ticket:"), newline)(s)?;
    parse_ticket(s)
}

fn parse_nearby_tickets(s: &str) -> IResult<&str, Vec<Ticket>> {
    let (s, _) = multispace0(s)?;
    let (s, _) = pair(tag("nearby tickets:"), newline)(s)?;
    many1(parse_ticket)(s)
}

fn parse(s: &str) -> IResult<&str, State> {
    let (s, fields) = parse_fields(s)?;
    let (s, ticket) = parse_mine_ticket(s)?;
    let (s, nearby) = parse_nearby_tickets(s)?;
    let (s, _) = multispace0(s)?;

    let state = State {
        fields,
        ticket,
        nearby,
    };
    Ok((s, state))
}

#[aoc_generator(day16)]
pub fn gen(input: &str) -> State {
    match parse(input) {
        Ok(("", state)) => state,
        _ => panic!("Parsing error")
    }
}

#[aoc(day16, part1)]
pub fn solve_part1(state: &State) -> usize {
    state.nearby
        .iter()
        .flat_map(|ticket| ticket.get_invalid_values(&state.fields))
        .sum()
}

fn filter_fields<'a>(vals: &[usize], fields: &'a [Field]) -> Vec<&'a Field> {
    fields
        .iter()
        .filter(|f| {
            vals
                .iter()
                .all(|v| f.in_bounds(v))
        })
        .collect()
}

fn find_fields_names<'a>(fields: &[(usize, Vec<&'a Field>)]) -> Vec<(usize, &'a Field)> {
    let mut ret: Vec<(usize, &Field)> = Vec::new();
    let mut previous: HashSet<String> = HashSet::new();

    for (i, f) in fields {
        let field = f
            .iter()
            .filter(|f| previous.contains(&f.name[..]).not())
            .collect_vec();

        assert_eq!(field.len(), 1);
        previous.insert(field[0].name.clone());
        ret.push((*i, field[0]));
    }

    ret
}

#[aoc(day16, part2)]
pub fn solve_part2(state: &State) -> usize {
    let nearby = state.nearby
        .iter()
        .filter(|ticket| ticket.is_valid(&state.fields))
        .cloned()
        .collect();
    let state = State {
        nearby,
        ..state.clone()
    };

    let columns = state.get_columns();
    let columns_fields = columns
        .iter()
        .map(|c| filter_fields(c, &state.fields))
        .enumerate()
        .sorted_by(|(_, x), (_, y)| x.len().cmp(&y.len()))
        .collect_vec();
    let names = find_fields_names(&columns_fields);
    let indexes = names
        .iter()
        .filter(|(_, f)| f.name.starts_with("departure"))
        .map(|(i, _)| i)
        .collect_vec();

    let mut product = 1;
    for i in indexes {
        product *= state.ticket.values[*i];
    }

    product
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
        return "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
    }

    fn get_input2() -> &'static str {
        return "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";
    }

    #[test]
    fn test_gen() {
        println!("{:?}", gen(get_input()));
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&gen(get_input())), 71);
    }

    #[test]
    fn test_part2() {
        solve_part2(&gen(get_input2()));
    }
}
