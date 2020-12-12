#[derive(Debug)]
pub enum Dir {
    N(isize),
    S(isize),
    E(isize),
    W(isize),
    L(isize),
    R(isize),
    F(isize),
}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, Copy, Clone)]
pub struct State {
    face: isize,
    pos: Point,
    waypoint: Point,
}

impl Dir {
    fn from(s: &str) -> Option<Dir> {
        assert!(s.len() >= 2);
        Some(match (&s[0..1], &s[1..]) {
            ("N", n) => Dir::N(n.parse().ok()?),
            ("S", n) => Dir::S(n.parse().ok()?),
            ("E", n) => Dir::E(n.parse().ok()?),
            ("W", n) => Dir::W(n.parse().ok()?),
            ("L", n) => Dir::L(n.parse().ok()?),
            ("R", n) => Dir::R(n.parse().ok()?),
            ("F", n) => Dir::F(n.parse().ok()?),
            _ => unreachable!()
        })
    }
}

impl State {
    fn new() -> State {
        State {
            face: 0,
            pos: Point { x: 0, y: 0 },
            waypoint: Point { x: 10, y: -1 },
        }
    }

    fn forward(&self, n: isize) -> State {
        match self.face {
            0 => self.advance(&Dir::E(n)),
            90 => self.advance(&Dir::S(n)),
            180 => self.advance(&Dir::W(n)),
            270 => self.advance(&Dir::N(n)),
            _ => panic!("Something went wrong :p")
        }
    }

    fn advance(&self, dir: &Dir) -> State {
        match dir {
            Dir::N(n) => State { pos: Point { x: self.pos.x, y: self.pos.y - n }, ..*self },
            Dir::S(n) => State { pos: Point { x: self.pos.x, y: self.pos.y + n }, ..*self },
            Dir::E(n) => State { pos: Point { x: self.pos.x + n, y: self.pos.y }, ..*self },
            Dir::W(n) => State { pos: Point { x: self.pos.x - n, y: self.pos.y }, ..*self },
            Dir::L(n) => State { face: ((self.face + 360) - n) % 360, ..*self },
            Dir::R(n) => State { face: (self.face + n) % 360, ..*self },
            Dir::F(n) => self.forward(*n),
        }
    }

    fn turn_around(&self, dir: &Dir) -> Point {
        let (n, inc) = match dir {
            Dir::L(n) => (n, Point { x: 1, y: -1 }),
            Dir::R(n) => (n, Point { x: -1, y: 1 }),
            _ => unreachable!()
        };
        let n = n / 90;

        (0..n)
            .fold(self.waypoint, |point, _|
                Point { x: point.y * inc.x, y: point.x * inc.y },
            )
    }

    fn move_to(&self, n: isize) -> Point {
        Point {
            x: self.pos.x + (self.waypoint.x * n),
            y: self.pos.y + (self.waypoint.y * n),
        }
    }

    fn advance_waypoint(&self, dir: &Dir) -> State {
        match dir {
            Dir::N(n) => State { waypoint: Point { x: self.waypoint.x, y: self.waypoint.y - n }, ..*self },
            Dir::S(n) => State { waypoint: Point { x: self.waypoint.x, y: self.waypoint.y + n }, ..*self },
            Dir::E(n) => State { waypoint: Point { x: self.waypoint.x + n, y: self.waypoint.y }, ..*self },
            Dir::W(n) => State { waypoint: Point { x: self.waypoint.x - n, y: self.waypoint.y }, ..*self },
            Dir::L(_) | Dir::R(_) => State { waypoint: self.turn_around(dir), ..*self },
            Dir::F(n) => State { pos: self.move_to(*n), ..*self },
        }
    }

    fn distance(&self) -> usize {
        self.pos.x.abs() as usize + self.pos.y.abs() as usize
    }
}

#[aoc_generator(day12)]
pub fn gen(input: &str) -> Vec<Dir> {
    input
        .lines()
        .flat_map(|line| Dir::from(line))
        .collect()
}

#[aoc(day12, part1)]
pub fn solve_part1(dirs: &Vec<Dir>) -> usize {
    dirs
        .iter()
        .fold(State::new(), |state, dir| state.advance(dir))
        .distance()
}

#[aoc(day12, part2)]
pub fn solve_part2(dirs: &Vec<Dir>) -> usize {
    dirs
        .iter()
        .fold(State::new(), |state, dir| state.advance_waypoint(dir))
        .distance()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
        return "F10
N3
F7
R90
F11";
    }

    #[test]
    fn test_gen() {
        println!("{:?}", gen(get_input()));
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&gen(get_input())), 25);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&gen(get_input())), 286);
    }
}
