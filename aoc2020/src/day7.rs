use std::collections::HashMap;
use itertools::Itertools;

#[derive(Debug)]
pub struct Bag {
    name: String,
    contains: Vec<(usize, String)>,
}

impl Bag {
    fn get_content(colors: Vec<&str>) -> Vec<(usize, String)> {
        colors
            .iter()
            .map(|&s| {
                let n: String = s.chars().take_while(|&c| c != ' ').collect();
                let n: usize = n.parse().expect("Expected a number");
                let t: String = s.chars().skip_while(|&c| c != ' ').skip(1).collect();
                (n, t)
            })
            .collect()
    }

    pub fn from(s: &str) -> Bag {
        s
            .replace(" bags contain", ",")
            .replace(" bags", "")
            .replace(" bag", "")
            .replace(", no other", "")
            .replace('.', "")
            .split(", ")
            .collect_vec()
            .as_slice()
            .split_first()
            .map(|(&head, tail)|
                Bag {
                    name: head.to_string(),
                    contains: Bag::get_content(tail.to_vec()),
                }
            )
            .expect("Invalid line")
    }
}

#[aoc_generator(day7)]
pub fn gen(input: &str) -> HashMap<String, Bag> {
    input
        .lines()
        .map(|line| Bag::from(line))
        .map(|bag| (bag.name.clone(), bag))
        .collect()
}

fn can_contain(color: &str, bags: &HashMap<String, Bag>) -> bool {
    bags[color].contains
        .iter()
        .any(|(_, col)| col == "shiny gold" || can_contain(col, bags))
}

fn count(color: &str, bags: &HashMap<String, Bag>) -> usize {
    1 + bags[color].contains
        .iter()
        .map(|(size, col)| size * count(col, bags))
        .sum::<usize>()
}

#[aoc(day7, part1)]
pub fn solve_part1(bags: &HashMap<String, Bag>) -> usize {
    bags
        .keys()
        .filter(|&color| color != "shiny gold" && can_contain(color, bags))
        .count()
}

#[aoc(day7, part2)]
pub fn solve_part2(bags: &HashMap<String, Bag>) -> usize {
    bags["shiny gold"].contains
        .iter()
        .map(|(size, col)| size * count(col, bags))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
        return "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
    }

    fn get_input2() -> &'static str {
        return "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
    }

    #[test]
    pub fn test_bag_from() {
        // get_input()
        get_input2()
            .lines()
            .for_each(|line| println!("{:?}", Bag::from(line)));

        // let bags: HashMap<String, Bag> = get_input()
        //     .lines()
        //     .map(|line| Bag::from(line))
        //     .map(|bag| (bag.name.clone(), bag))
        //     .collect();
        //
        // println!("{:?}", bags);
    }

    #[test]
    pub fn test_part1() {
        assert_eq!(solve_part1(&gen(get_input())), 4);
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(solve_part2(&gen(get_input())), 32);
        assert_eq!(solve_part2(&gen(get_input2())), 126);
    }
}