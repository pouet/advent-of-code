pub struct Point {
    h: i32,
    d: i32,
    a: i32,
}

#[derive(Debug)]
pub enum Dir {
    Up(i32),
    Down(i32),
    Forward(i32),
}

impl Dir {
    pub fn from(s: &str) -> Dir {
        let sp: Vec<_> = s.split(' ').collect();
        let dir = sp.get(0).map(|&s| s);
        let n = sp.get(1).map(|&s| s.parse::<i32>().ok()).flatten();

        match (dir, n) {
            (Some("up"), Some(n)) => Dir::Up(n),
            (Some("down"), Some(n)) => Dir::Down(n),
            (Some("forward"), Some(n)) => Dir::Forward(n),
            (_, _) => panic!("Wrong input")
        }
    }
}

#[aoc_generator(day2)]
pub fn gen(input: &str) -> Vec<Dir> {
    input
        .lines()
        .map(|s| Dir::from(s))
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Dir]) -> i32 {
    let p = input.iter()
        .fold(Point { h: 0, d: 0, a: 0 }, |p, dir| {
            match dir {
                Dir::Up(n) => Point { d: p.d - n, ..p },
                Dir::Down(n) => Point { d: p.d + n, ..p },
                Dir::Forward(n) => Point { h: p.h + n, ..p },
            }
        });
    p.d * p.h
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Dir]) -> i32 {
    let p = input.iter()
        .fold(Point { h: 0, d: 0, a: 0 }, |p, dir| {
            match dir {
                Dir::Up(n) => Point { a: p.a - n, ..p },
                Dir::Down(n) => Point { a: p.a + n, ..p },
                Dir::Forward(n) => Point { h: p.h + n, d: p.d + (p.a * n), ..p },
            }
        });
    p.d * p.h
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_gen() {
        println!("{:?}", gen(INPUT));
    }

    #[test]
    fn test_part1() {
        let input = gen(INPUT);
        assert_eq!(solve_part1(&input), 150);
    }

    #[test]
    fn test_part2() {
        let input = gen(INPUT);
        assert_eq!(solve_part2(&input), 900);
    }
}