use itertools::{Itertools, MinMaxResult};
use std::ops::Not;

#[aoc_generator(day9)]
pub fn gen(input: &str) -> Vec<usize> {
    input
        .lines()
        .flat_map(|line| line.parse())
        .collect()
}

pub fn part1(input: &[usize], preamble: usize) -> Option<usize> {
    let windows = input.windows(preamble);

    input
        .iter()
        .skip(preamble)
        .zip(windows)
        .find(|(&x, win)|
            win
                .iter()
                .permutations(2)
                .any(|p| p[0] + p[1] == x)
                .not()
        )
        .map(|(&n, _)| n)
}

// fn fold_while(input: &[&usize], target: &usize) -> (usize, usize) {
//     fn inner(input: &[&usize], target: &usize, acc: usize, index: usize) -> (usize, usize) {
//         match input.split_first() {
//             Some((&x, tail)) if acc < *target => inner(tail, target, acc + x, index + 1),
//             _ => (acc, index)
//         }
//     }
//
//     inner(input, target, 0, 0)
// }

// trait FoldWhile: Iterator {
//     type Item;
//
//     fn yolo<Acc, Func, Cond>(self, init: Acc, f: Func, cond: Cond) -> Acc
//         where
//             Func: FnMut(Acc, Self::Item) -> Acc,
//             Cond: FnMut(&Acc) -> bool,
//     ;
// }
//
// impl<I> FoldWhile for I
//     where I: Iterator// + Iterator<Item=I>
// {
//     type Item = Self;
//
//     fn yolo<Acc, Func, Cond>(mut self, init: Acc, mut f: Func, mut cond: Cond) -> Acc
//         where
//             Func: FnMut(Acc, Self::Item) -> Acc,
//             Cond: FnMut(&Acc) -> bool,
//     {
//     let mut acc = init;
//         while let Some(x) = self.next() {
//             if cond(&acc) {
//                 break;
//             }
//             acc = f(acc, x);
//         }
//         acc
//     }
// }


pub fn part2(input: &[usize], target: usize) -> Option<usize> {
    // let v= vec![0];
    // v.iter()
    //     .yolo(0, |(_, _)| 0, |_| true);
    input
        .iter()
        .enumerate()
        .find_map(|(i, _)| {
            let (y, acc) = input
                .iter()
                .skip(i)
                .enumerate()
                // .yolo((0, 0), |(i, acc), (j, it)| (i, acc), |_| true)
                // need a fold_while :(
                .fold((0, 0), |(i, acc), (j, it)|
                    if acc < target { (j, acc + *it) } else { (i, acc) });

            if acc == target {
                match input[i..i + y].iter().minmax() {
                    MinMaxResult::MinMax(min, max) => Some(min + max),
                    _ => unreachable!()
                }
            } else {
                None
            }
        })
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[usize]) -> Option<usize> {
    part1(input, 25)
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[usize]) -> Option<usize> {
    part2(input, 41682220)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
        return "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
    }

    #[test]
    fn test_gen() {
        println!("{:?}", gen(&get_input()));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&gen(get_input()), 5), Some(127));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&gen(get_input()), 127), Some(62));
    }
}