// use nom::lib::std::collections::HashMap;
use std::ops::Not;
use nom::lib::std::mem::size_of_val;

fn solve(nums: &[usize], target: usize) -> usize {
    let mut tab = vec![None; target];

    assert!(nums.is_empty().not());

    nums
        .iter()
        .enumerate()
        .for_each(|(turn, n)| {
            tab[*n] = Some(turn + 1);
        });

    let mut last = *nums.last().unwrap();
    for turn in nums.len()..target {
        let v = tab[last];
        tab[last] = Some(turn);
        last = match v {
            None => 0,
            Some(n) => turn - n
        };
    }

    last
}

#[aoc_generator(day15)]
pub fn gen(input: &str) -> Vec<usize> {
    input
        .split(',')
        .flat_map(|n| n.parse().ok())
        .collect()
}

#[aoc(day15, part1)]
pub fn solve_part1(nums: &[usize]) -> usize {
    solve(nums, 2020)
}

#[aoc(day15, part2)]
pub fn solve_part2(nums: &[usize]) -> usize {
    solve(nums, 30000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
        return "0,3,6";
    }

    #[test]
    fn test_gen() {
        println!("{:?}", gen(get_input()));
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&gen("0,3,6")), 436);
        assert_eq!(solve_part1(&gen("0,3,6")), 436);
        assert_eq!(solve_part1(&gen("1,3,2")), 1);
        assert_eq!(solve_part1(&gen("2,1,3")), 10);
        assert_eq!(solve_part1(&gen("1,2,3")), 27);
        assert_eq!(solve_part1(&gen("2,3,1")), 78);
        assert_eq!(solve_part1(&gen("3,2,1")), 438);
        assert_eq!(solve_part1(&gen("3,1,2")), 1836);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&gen("0,3,6")), 175594);
        // assert_eq!(solve_part2(&gen("1,3,2")), 2578);
        // assert_eq!(solve_part2(&gen("2,1,3")), 3544142);
        // assert_eq!(solve_part2(&gen("1,2,3")), 261214);
        // assert_eq!(solve_part2(&gen("2,3,1")), 6895259);
        // assert_eq!(solve_part2(&gen("3,2,1")), 18);
        // assert_eq!(solve_part2(&gen("3,1,2")), 362);
    }
}

// type MapType = HashMap<usize, usize>;
//
// fn solve(nums: &[usize], target: usize) -> usize {
//     let mut h: MapType = HashMap::new();
//
//     assert!(nums.is_empty().not());
//
//     nums
//         .iter()
//         .enumerate()
//         .for_each(|(turn, n)| {
//             h.insert(*n, turn + 1);
//         });
//
//     let mut last = *nums.last().unwrap();
//     for turn in nums.len()..target {
//         last = match h.insert(last, turn - 0) {
//             Some(n) => turn - n,
//             None => 0
//         };
//     }
//
//     last
// }
