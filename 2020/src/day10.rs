use itertools::Itertools;
use nom::lib::std::collections::HashMap;

#[aoc_generator(day10)]
pub fn gen(input: &str) -> Vec<usize> {
    input
        .lines()
        .flat_map(|line| line.parse().ok())
        .sorted()
        .collect()
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[usize]) -> Option<usize> {
    let (n1, _, n3, _) = input
        .iter()
        .fold((0, 0, 1, 0), |(n1, n2, n3, acc), n| {
            match n.checked_sub(acc) {
                Some(1) => (n1 + 1, n2, n3, acc + 1),
                Some(2) => (n1, n2 + 1, n3, acc + 2),
                Some(3) => (n1, n2, n3 + 1, acc + 3),
                _ => panic!("Invalid input")
            }
        });

    Some(n1 * n3)
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &[usize]) -> Option<usize> {
    let mut map: HashMap<usize, usize> = HashMap::new();
    map.insert(0, 1);

    for &n in input.iter() {
        let x = (1..=3)
            .fold(0, |acc, i|
                acc + n.checked_sub(i)
                    .map(|i| *map.get(&i).unwrap_or(&0usize))
                    .unwrap_or(0),
            );
        map.insert(n, x);
    }

    input.iter().last().map(|max| map[max])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input1() -> &'static str {
        return "16
10
15
5
1
11
7
19
6
12
4";
    }

    fn get_input2() -> &'static str {
        return "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
    }

    #[test]
    fn test_gen() {
        println!("{:?}", gen(get_input1()));
        println!("{:?}", gen(get_input2()));
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&gen(get_input1())), Some(35));
        assert_eq!(solve_part1(&gen(get_input2())), Some(220));
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&gen(get_input1())), Some(8));
        assert_eq!(solve_part2(&gen(get_input2())), Some(19208));
    }
}