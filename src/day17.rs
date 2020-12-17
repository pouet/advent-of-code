#[aoc_generator(day14)]
pub fn gen(input: &str) -> usize {
    0
}

#[aoc(day14, part1)]
pub fn solve_part1(state: &usize) -> usize {
    *state
}

#[aoc(day14, part2)]
pub fn solve_part2(state: &usize) -> usize {
    *state
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
        return "";
    }

    #[test]
    fn test_gen() {
        println!("{:?}", gen(get_input()));
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&gen(get_input())), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&gen(get_input())), 0);
    }
}