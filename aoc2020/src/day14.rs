use nom::bytes::complete::tag;
use nom::sequence::{separated_pair, delimited, pair, terminated, tuple};
use nom::character::complete::{digit1, multispace0, alphanumeric1, newline};
use nom::multi::many1;
use nom::IResult;
use nom::combinator::map;
use nom::lib::std::collections::HashMap;
use itertools::Itertools;
use std::ops::Not;

#[derive(Debug, Copy, Clone)]
pub struct Mask {
    bit: u64,
    value: u64,
    x: bool,
}

#[derive(Debug)]
pub struct State {
    masks: Vec<Mask>,
    mem: Vec<(u64, u64)>,
}

pub fn parse_mask(s: &str) -> IResult<&str, Vec<Mask>> {
    fn s_to_mask(s: &str) -> Vec<Mask> {
        s
            .chars()
            .rev()
            .enumerate()
            .fold(Vec::new(), |mut acc, (i, c)| {
                let n: u64 = c.to_digit(2).unwrap_or(0) as u64;
                acc.push(Mask { bit: i as u64, value: n, x: c == 'X' });
                acc
            })
    }

    let mask_str = tuple((tag("mask"), multispace0, tag("="), multispace0));
    let mask = pair(mask_str, terminated(alphanumeric1, newline));
    map(mask, |(_, s): (_, &str)| s_to_mask(s))(s)
}

pub fn parse_mem(s: &str) -> IResult<&str, Vec<(u64, u64)>> {
    let mem_n = delimited(
        tag("mem["),
        digit1,
        tag("]"),
    );
    let mem = separated_pair(
        mem_n,
        tag(" = "),
        digit1,
    );
    let (s, mems) = many1(pair(mem, multispace0))(s)?;

    let v = mems
        .iter()
        .map(|((i, val), _)|
            (i.parse().unwrap(), val.parse().unwrap())
        )
        .collect();

    Ok((s, v))
}

pub fn parse(s: &str) -> IResult<&str, Vec<State>> {
    let (s, mems) = many1(pair(parse_mask, parse_mem))(s)?;

    let mems = mems
        .iter()
        .map(|(masks, mem)| State { masks: masks.clone(), mem: mem.clone() })
        .collect();

    Ok((s, mems))
}

#[aoc_generator(day14)]
pub fn gen(input: &str) -> Vec<State> {
    match parse(input) {
        Ok((_, ret)) => ret,
        _ => panic!("Parsing error")
    }
}

trait BitUtils<T> {
    fn set_bit(&self, n: u64) -> Self;
    fn unset_bit(&self, n: u64) -> Self;
    fn set_bit_value(&self, n: u64, val: T) -> Self;
    fn get_bit(&self, n: u64) -> Self;
}

impl BitUtils<u64> for u64 {
    fn set_bit(&self, n: u64) -> Self {
        let mask: Self = 1 << n;
        self | mask
    }

    fn unset_bit(&self, n: u64) -> Self {
        let mask: Self = 1 << n;
        self & (!mask)
    }

    fn set_bit_value(&self, n: u64, val: u64) -> Self {
        assert!(val <= 1);
        let m = self.unset_bit(n);
        match val {
            0 => m,
            1 => m.set_bit(n),
            _ => unreachable!()
        }
    }

    fn get_bit(&self, n: u64) -> Self {
        let mask: Self = 1 << n;
        (self & (!mask)) >> n
    }
}

#[aoc(day14, part1)]
pub fn solve_part1(states: &[State]) -> u64 {
    fn apply_masks(masks: &[Mask], val: &u64) -> u64 {
        masks
            .iter()
            .fold(*val, |acc, mask|
                if mask.x.not() {
                    acc.set_bit_value(mask.bit, mask.value)
                } else {
                    acc
                },
            )
    }
    let mut h: HashMap<u64, u64> = HashMap::new();

    for state in states.iter() {
        for (i, val) in state.mem.iter() {
            let n = apply_masks(&state.masks, val);
            h.insert(*i, n);
        }
    }

    h.values().sum()
}

fn do_it_n_times(masks: &[Mask], value: u64) -> Vec<u64> {
    if masks.is_empty() {
        vec![value]
    } else {
        let mask = masks[0];
        let value = value.unset_bit(mask.bit);
        let mut v = do_it_n_times(&masks[1..], value);
        let value = value.set_bit(mask.bit);
        let mut w = do_it_n_times(&masks[1..], value);

        v.append(&mut w);
        v
    }
}

#[aoc(day14, part2)]
pub fn solve_part2(states: &[State]) -> u64 {
    pub fn apply_masks(masks: &[Mask], val: &u64) -> u64 {
        masks
            .iter()
            .fold(*val, |acc, mask|
                if mask.x.not() && mask.value == 1 {
                    acc.set_bit(mask.bit)
                } else {
                    acc
                },
            )
    }
    let mut h: HashMap<u64, u64> = HashMap::new();

    for state in states.iter() {
        let floating = state.masks.iter().filter(|m| m.x).copied().collect_vec();
        for (i, val) in state.mem.iter() {
            let n = apply_masks(&state.masks, i);
            do_it_n_times(&floating, n)
                .iter()
                .for_each(|x| { let _ = h.insert(*x, *val); });
        }
    }

    h.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
        return "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
    }

    fn get_input1() -> &'static str {
        return "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
    }

    #[test]
    fn test_gen() {
        // let state = gen(get_input());
        // println!("state: {:?}", state);
    }

    #[test]
    fn test_bits() {
        assert_eq!(0.set_bit(0), 0b1);
        assert_eq!(0.set_bit(1), 0b10);
        assert_eq!(0.set_bit(32), 0b100000000000000000000000000000000);

        assert_eq!(1.unset_bit(0), 0b0);
        assert_eq!(0b11.unset_bit(1), 0b1);
        assert_eq!(0b100000000000000000000000000000001.unset_bit(32), 0b1);

        assert_eq!(0.set_bit_value(0, 1), 0b1);
        assert_eq!(0.set_bit_value(0, 0), 0b0);
        assert_eq!(1.set_bit_value(0, 1), 0b1);
        assert_eq!(1.set_bit_value(0, 0), 0b0);
        assert_eq!(0b000001.set_bit_value(5, 1), 0b100001);
        assert_eq!(0b000001.set_bit_value(5, 0), 0b000001);
        assert_eq!(0b100001.set_bit_value(5, 1), 0b100001);
        assert_eq!(0b100001.set_bit_value(5, 0), 0b000001);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&gen(get_input())), 165);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&gen(get_input1())), 208);
    }
}