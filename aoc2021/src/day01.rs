#[aoc_generator(day1)]
pub fn gen(input: &str) -> Vec<i32> {
    input
        .lines()
        .flat_map(|s| s.parse::<i32>())
        .collect()
}

fn count_increased(input: &[i32]) -> usize {
    input
        .windows(2)
        .filter(|n| { n[0] < n[1] })
        .count()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i32]) -> usize {
    count_increased(input)
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i32]) -> usize {
    let tmp = input
        .windows(3)
        .map(|n| n.iter().sum::<i32>())
        .collect::<Vec<i32>>();
    count_increased(&tmp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen() {
        println!("{:?}", gen("199\n200\n208\n210\n200\n207\n240\n269\n260\n263"));
    }

    #[test]
    fn test_part1() {
        let input = gen("199\n200\n208\n210\n200\n207\n240\n269\n260\n263");
        assert_eq!(solve_part1(&input), 7);
    }

    #[test]
    fn test_part2() {
        let input = gen("199\n200\n208\n210\n200\n207\n240\n269\n260\n263");
        assert_eq!(solve_part2(&input), 5);
    }
}