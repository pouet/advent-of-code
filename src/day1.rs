use itertools::Itertools;

#[aoc_generator(day1)]
pub fn gen(input: &str) -> Vec<i32> {
    input
        .lines()
        .flat_map(|s| s.parse::<i32>())
        .collect()
}

fn solve(input: &Vec<i32>, chunk: usize) -> Option<i32> {
    return input
        .iter()
        .combinations(chunk)
        .find(|v| v
            .iter()
            .map(|&n| *n)
            .sum::<i32>() == 2020)
        .map(|v| v
            .iter()
            .map(|&n| *n)
            .product());
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Vec<i32>) -> Option<i32> {
    return solve(input, 2);
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Vec<i32>) -> Option<i32> {
    return solve(input, 3);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<i32> {
        return gen("1721\n979\n366\n299\n675\n1456");
    }

    #[test]
    fn test1() {
        assert_eq!(solve(&get_input(), 2), Some(514579));
    }

    #[test]
    fn test2() {
        assert_eq!(solve(&get_input(), 3), Some(241861950));
    }
}