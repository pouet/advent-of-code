pub fn count_one_in_a_column(input: &[Vec<u32>], column: usize) -> usize {
    input.iter()
        .filter(|line| line[column] == 1)
        .count()
}

pub fn life_support_rating(input: &[Vec<u32>], base_value: u32) -> u32 {
    let len = input[0].len();
    let mut copy = input.iter().cloned().collect::<Vec<Vec<_>>>();

    for i in 0..len {
        let count = count_one_in_a_column(&copy, i);
        let bit = if count >= (copy.len() - count) { 1 - base_value } else { base_value };
        copy = copy.into_iter().filter(|line| line[i] == bit).collect();
        if copy.len() == 1 {
            break
        }
    }


    copy.first().map(|a| a.fold(0, |acc, &n| (acc << 1) | n)).unwrap()
}

#[aoc_generator(day3)]
pub fn gen(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|s| s.chars().flat_map(|c| c.to_digit(10)).collect::<Vec<_>>())
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Vec<u32>]) -> u32 {
    let len = input[0].len();
    let mid = input.len() / 2;
    let mut n = 0;

    for i in 0..len {
        let count = count_one_in_a_column(&input, i);
        let bit = if count > mid { 1 } else { 0 };
        n = (n << 1) | bit;
    }

    let mask = (!0) >> (u32::BITS - len as u32);
    n * ((!n) & mask)
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Vec<u32>]) -> u32 {
    let res1 = life_support_rating(input, 0);
    let res2 = life_support_rating(input, 1);

    res1 * res2
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_gen() {
        println!("{:?}", gen(INPUT));
    }

    #[test]
    fn test_part1() {
        let input = gen(INPUT);
        assert_eq!(solve_part1(&input), 198);
    }

    #[test]
    fn test_part2() {
        let input = gen(INPUT);
        assert_eq!(solve_part2(&input), 230);
    }
}