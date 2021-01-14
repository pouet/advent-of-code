use itertools::Itertools;
use std::ops::Not;

#[derive(Debug, Clone)]
pub struct Crab {
    current: usize,
    cups: Vec<usize>,
}

impl Crab {
    fn from(s: &str) -> Crab {
        let nums = s
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect_vec();

        let mut cups = vec![0; nums.len() + 1];
        for (i, cup) in nums.iter().enumerate() {
            cups[*cup] = nums[(i + 1) % nums.len()];
        }

        Crab {
            current: nums[0],
            cups,
        }
    }

    fn play(&mut self) {
        let current = self.current;
        let n1 = self.cups[current];
        let n2 = self.cups[n1];
        let n3 = self.cups[n2];
        self.cups[current] = self.cups[n3];

        let mut dest = current;
        loop {
            dest = dest - 1;
            if dest == 0 {
                dest = self.cups.len() - 1;
            }
            if [n1, n2, n3].contains(&dest).not() {
                break;
            }
        }

        self.cups[n3] = self.cups[dest];
        self.cups[dest] = n1;
        self.current = self.cups[self.current];
    }
}


#[aoc_generator(day23)]
pub fn gen(input: &str) -> Crab {
    Crab::from(input)
}

#[aoc(day23, part1)]
pub fn solve_part1(crab: &Crab) -> usize {
    let mut crab = crab.clone();
    for _ in 0..100 {
        crab.play();
    }

    let mut cur = 1;
    let mut ret = 0;
    while crab.cups[cur] != 1 {
        ret = (ret * 10) + crab.cups[cur];
        cur = crab.cups[cur];
    }

    ret
}

#[aoc(day23, part2)]
pub fn solve_part2(crab: &Crab) -> usize {
    let mut crab = crab.clone();
    let pos = crab.cups.iter().position(|&x| x == crab.current).unwrap();

    crab.cups[pos] = crab.cups.len();
    for i in crab.cups.len()..1000000 {
        crab.cups.push(i + 1);
    }
    crab.cups.push(crab.current);

    for _ in 0..10000000 {
        crab.play();
    }

    crab.cups[1] * crab.cups[crab.cups[1]]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
        return "389125467";
    }

    #[test]
    fn test_gen() {
        println!("{:?}", gen(get_input()));
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&gen(get_input())), 67384529);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&gen(get_input())), 149245887792);
    }
}
