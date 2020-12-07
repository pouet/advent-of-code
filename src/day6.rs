use itertools::Itertools;

#[aoc_generator(day6)]
pub fn gen(input: &str) -> Vec<String> {
    input
        .split("\n\n")
        .map(|s| s.to_string())
        .collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &Vec<String>) -> usize {
    input
        .iter()
        .map(|s| s
            .replace('\n', "")
            .chars()
            .unique()
            .count())
        .sum()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &Vec<String>) -> usize {
    input
        .iter()
        .map(|line| {
            // 1
            let persons = line.split('\n').count();
            line
                .replace('\n', "").chars().sorted().collect_vec().iter()
                .group_by(|x| *x).into_iter().map(|(_, group)| group.collect::<Vec<&char>>())
                .filter(|group| group.len() == persons)
                .count()

            // 2
            // line
            //     .split('\n')
            //     .map(|s| s.chars().collect_vec())
            //     .collect_vec()
            //     .split_first()
            //     .map(|(head, tail)| head
            //         .iter()
            //         .filter(|c| tail
            //             .iter()
            //             .all(|v| v.contains(c))
            //         )
            //         .collect_vec()
            //     )
            //     .unwrap()
            //     .len()

            // 3
            // let persons = line.split('\n').count();
            // let mut count = [0; 128];
            // for c in line.chars() {
            //     count[c as usize] += 1;
            // }
            // count
            //     .iter()
            //     .filter(|&&n| n == persons)
            //     .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
        return "abc

a
b
c

ab
ac

a
a
a
a

b";
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&gen(get_input())), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&gen(get_input())), 6);
    }
}