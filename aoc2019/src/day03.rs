use std::collections::{HashSet, HashMap};

#[derive(Debug, Copy, Clone)]
pub enum Dir {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, Copy, Clone)]
pub struct Item {
    direction: Dir,
    distance: i32,
}

type Path = Vec<Item>;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan(&self, p2: &Point) -> i32 {
        let a = (p2.x - self.x).abs();
        let b = (p2.y - self.y).abs();
        a + b
    }

    fn move_to(&mut self, dir: &Dir) {
        self.x += dir.get_x();
        self.y += dir.get_y();
    }
}

impl Item {
    fn from(s: &str) -> Result<Item, &'static str> {
        Ok(Item {
            direction: Dir::from(&s[..1])?,
            distance: s[1..].parse().map_err(|_| "Expected a valid number")?,
        })
    }
}

impl Dir {
    fn from(s: &str) -> Result<Dir, &'static str> {
        Ok(match s {
            "R" => Dir::Right,
            "L" => Dir::Left,
            "U" => Dir::Up,
            "D" => Dir::Down,
            _ => Err("Invalid direction")?
        })
    }

    fn get_x(&self) -> i32 {
        match self {
            Dir::Right => 1,
            Dir::Left => -1,
            Dir::Up => 0,
            Dir::Down => 0,
        }
    }

    fn get_y(&self) -> i32 {
        match self {
            Dir::Right => 0,
            Dir::Left => 0,
            Dir::Up => 1,
            Dir::Down => -1,
        }
    }
}

fn draw_wire(path: &Path) -> HashMap<Point, i32> {
    let mut h: HashMap<Point, i32> = HashMap::new();
    let mut p = Point { x: 0, y: 0 };
    let mut i = 1;

    for it in path {
        for _ in 0..it.distance {
            p.move_to(&it.direction);
            h.entry(p).or_insert(i);
            i += 1;
        }
    }

    h
}

#[aoc_generator(day3)]
pub fn gen(input: &str) -> Result<(Path, Path), &'static str> {
    let paths = input
        .lines()
        .map(|line| line
            .split(',')
            .map(Item::from)
            .collect())
        .collect::<Result<Vec<Path>, _>>()?;

    match (paths.get(0), paths.get(1)) {
        (Some(p1), Some(p2)) => Ok((p1.clone(), p2.clone())),
        _ => Err("Ups retard"),
    }
}

#[aoc(day3, part1)]
pub fn solve_part1(paths: &(Path, Path)) -> Option<i32> {
    let point0: HashSet<Point> = draw_wire(&paths.0).keys().cloned().collect();
    let point1: HashSet<Point> = draw_wire(&paths.1).keys().cloned().collect();
    let start = Point { x: 0, y: 0 };

    point0.intersection(&point1)
        .map(|p: &Point| p.manhattan(&start))
        .min()
}

#[aoc(day3, part2)]
pub fn solve_part2(paths: &(Path, Path)) -> Option<i32> {
    let point0 = draw_wire(&paths.0);
    let point1 = draw_wire(&paths.1);

    point0.iter()
        .filter_map(|(key0, dist0)|
            point1.get(key0).map(|dist1| dist0 + dist1))
        .min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen() {
        println!("{:?}", gen("R8,U5,L5,D3"));
        println!("{:?}", gen("Ua,R6,D4,L4"));
        println!("{:?}", gen("r8,U5,L5,D3"));
        println!("{:?}", gen("R8,U5,L5,D3\nU7,R6,D4,L4"));
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&gen("R8,U5,L5,D3\nU7,R6,D4,L4").unwrap()), Some(6));
        assert_eq!(solve_part1(&gen("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83").unwrap()), Some(159));
        assert_eq!(solve_part1(&gen("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7").unwrap()), Some(135));
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&gen("R8,U5,L5,D3\nU7,R6,D4,L4").unwrap()), Some(30));
        assert_eq!(solve_part2(&gen("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83").unwrap()), Some(610));
        assert_eq!(solve_part2(&gen("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7").unwrap()), Some(410));
    }
}