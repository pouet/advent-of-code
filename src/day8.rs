use nom::IResult;
use nom::character::complete::{one_of, digit1, space1};
use nom::combinator::{recognize, map_res, opt};
use nom::sequence::{pair, separated_pair};
use nom::branch::alt;
use nom::bytes::complete::tag;

#[derive(Debug, Clone)]
pub enum Op {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

pub struct State {
    ops: Vec<Op>,
    acc: isize,
    ip: usize,
}

impl State {
    pub fn new(ops: &Vec<Op>) -> State {
        State {
            ops: ops.clone(),
            acc: 0,
            ip: 0,
        }
    }

    fn increment_ip(&mut self, inc: isize) {
        self.ip = (self.ip as isize + inc) as usize;
    }

    fn op_acc(&mut self, n: isize) {
        self.acc += n;
        self.increment_ip(1);
    }

    fn op_jmp(&mut self, n: isize) {
        self.increment_ip(n);
    }

    fn op_nop(&mut self) {
        self.increment_ip(1);
    }

    fn is_finished(&self) -> bool {
        self.ip >= self.ops.len()
    }

    pub fn update(&mut self) {
        if !self.is_finished() {
            match self.ops[self.ip] {
                Op::Acc(n) => { self.op_acc(n); }
                Op::Jmp(n) => { self.op_jmp(n); }
                Op::Nop(_) => { self.op_nop(); }
            }
        }
    }
}

impl Op {
    fn from(s: &str) -> Option<Op> {
        fn parse_isize(s: &str) -> IResult<&str, isize> {
            let parser = pair(opt(one_of("+-")), digit1);
            map_res(recognize(parser), |s: &str| s.parse())(s)
        }

        let op = separated_pair(
            alt((tag("acc"), tag("jmp"), tag("nop"))),
            space1,
            parse_isize)(s);

        match op {
            Ok(("", ("nop", n))) => Some(Op::Nop(n)),
            Ok(("", ("jmp", n))) => Some(Op::Jmp(n)),
            Ok(("", ("acc", n))) => Some(Op::Acc(n)),
            _ => None
        }
    }
}

pub fn search_valid(inst: &Vec<Op>) -> Option<isize> {
    let mut exec = vec![false; inst.len()];
    let mut state = State::new(inst);

    loop {
        if state.is_finished() {
            return Some(state.acc);
        } else if exec[state.ip] {
            return None;
        }
        exec[state.ip] = true;
        state.update();
    }
}

#[aoc_generator(day8)]
pub fn gen(input: &str) -> Vec<Op> {
    input
        .lines()
        .map(|line| Op::from(line).expect("Invalid line"))
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(inst: &Vec<Op>) -> isize {
    let mut exec = vec![false; inst.len()];
    let mut state = State::new(inst);

    loop {
        if exec[state.ip] {
            return state.acc;
        }
        exec[state.ip] = true;
        state.update();
    }
}

#[aoc(day8, part2)]
pub fn solve_part2(inst: &Vec<Op>) -> Option<isize> {
    for (i, op) in inst.iter().enumerate() {
        let mut other = inst.to_vec();
        match op {
            Op::Jmp(n) => other[i] = Op::Nop(*n),
            Op::Nop(n) => other[i] = Op::Jmp(*n),
            _ => continue
        }
        if let Some(n) = search_valid(&other) {
            return Some(n);
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
        return "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    }

    #[test]
    fn test_nom() {
        get_input()
            .lines()
            .for_each(|s| println!("{:?}", Op::from(s)));
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&gen(get_input())), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&gen(get_input())), Some(8));
    }
}