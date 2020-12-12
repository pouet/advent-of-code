use nom::lib::std::fmt::Formatter;
use core::fmt;
use std::ops::Not;

#[derive(Clone, PartialEq)]
pub enum Seat {
    Floor,
    Empty,
    Occupied,
}

type Layout = Vec<Vec<Seat>>;

pub struct State {
    seats: Layout,
    height: usize,
    width: usize,
    changes: usize,
}

pub struct Position {
    x: isize,
    y: isize,
}

pub enum Direction {
    Up,
    Down
}

impl Iterator for Direction {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

impl fmt::Debug for Seat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Seat::Floor => write!(f, "."),
            Seat::Empty => write!(f, "L"),
            Seat::Occupied => write!(f, "#"),
        }
    }
}

impl Seat {
    fn from(c: char) -> Seat {
        match c {
            '.' => Seat::Floor,
            'L' => Seat::Empty,
            '#' => Seat::Occupied,
            _ => panic!("Invalid letter: {}", c)
        }
    }
}

impl State {
    fn new(seats: Layout) -> State {
        let height = seats.len();
        let width = seats[0].len();

        State {
            seats,
            height,
            width,
            changes: 0,
        }
    }

    fn next_seat1(&self, pos: Position) -> Seat {
        let width = self.width as isize;
        let height = self.height as isize;
        let in_bounds = |x, y| x >= 0 && x < width && y >= 0 && y < height;
        let count =
            (-1..=1).fold(0, |acc, x| {
                (-1..=1).fold(0, |acc, y| {
                    let p = Position { x: pos.x + x, y: pos.y + y };
                    let count = if (x == 0 && y == 0).not() && in_bounds(p.x, p.y) &&
                        self.seats[p.y as usize][p.x as usize] == Seat::Occupied {
                        1
                    } else {
                        0
                    };
                    acc + count
                }) + acc
            });

        match &self.seats[pos.y as usize][pos.x as usize] {
            Seat::Empty if count == 0 => Seat::Occupied,
            Seat::Occupied if count >= 4 => Seat::Empty,
            seat => seat.clone()
        }
    }

    fn next_seatn(&self, pos: Position) -> Seat {
        let width = self.width as isize;
        let height = self.height as isize;
        let in_bounds = |x, y| x >= 0 && x < width && y >= 0 && y < height;
        let dirs = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1), (0, 1),
            (1, -1), (1, 0), (1, 1)
        ];

        let mut count = 0;

        for (xdir, ydir) in dirs.iter() {
            let mut p = Position { x: pos.x + xdir, y: pos.y + ydir };
            while in_bounds(p.x, p.y) &&
                self.seats[p.y as usize][p.x as usize] == Seat::Floor {
                p = Position { x: p.x + xdir, y: p.y + ydir };
            }

            if in_bounds(p.x, p.y) &&
                self.seats[p.y as usize][p.x as usize] == Seat::Occupied {
                count += 1;
            }

        }
        match &self.seats[pos.y as usize][pos.x as usize] {
            Seat::Empty if count == 0 => Seat::Occupied,
            Seat::Occupied if count >= 5 => Seat::Empty,
            seat => seat.clone()
        }
    }

    fn update(&self) -> State {
        let mut seats: Layout = self.seats.to_vec();
        let changes =
            (0..self.height).fold(0, |acc, y|
                (0..self.width).fold(0, |acc, x| {
                    let p = Position { x: x as isize, y: y as isize };
                    let seat = self.next_seat1(p);
                    let change = if seat != seats[y][x] { 1 } else { 0 };
                    seats[y][x] = seat;
                    acc + change
                }) + acc,
            );

        State {
            seats,
            changes,
            ..*self
        }
    }

    fn updaten(&self) -> State {
        let mut seats: Layout = self.seats.to_vec();
        let changes =
            (0..self.height).fold(0, |acc, y|
                (0..self.width).fold(0, |acc, x| {
                    let p = Position { x: x as isize, y: y as isize };
                    let seat = self.next_seatn(p);
                    let change = if seat != seats[y][x] { 1 } else { 0 };
                    seats[y][x] = seat;
                    acc + change
                }) + acc,
            );

        State {
            seats,
            changes,
            ..*self
        }
    }

    fn count_occupied(&self) -> usize {
        self.seats.iter().fold(0, |acc, x|
            acc + x.iter().fold(0, |acc, y|
                acc + if *y == Seat::Occupied { 1 } else { 0 }))
    }
}

#[aoc_generator(day11)]
pub fn gen(input: &str) -> State {
    let seats: Layout = input
        .lines()
        .map(|line| line
            .trim()
            .chars()
            .map(|c| Seat::from(c))
            .collect()
        )
        .collect();

    State::new(seats)
}

#[aoc(day11, part1)]
pub fn solve_part1(state: &State) -> usize {
    fn rec(state: &State) -> usize {
        let state = state.update();
        if state.changes == 0 {
            state.count_occupied()
        } else {
            rec(&state)
        }
    }

    rec(state)
}

#[aoc(day11, part2)]
pub fn solve_part2(state: &State) -> usize {
    fn rec(state: &State) -> usize {
        let state = state.updaten();
        if state.changes == 0 {
            state.count_occupied()
        } else {
            rec(&state)
        }
    }

    // for v in &state.seats {
    //     println!("{:?}", v);
    // }
    // println!("--------------------------------------------------");
    // let state = state.updaten();
    // for v in &state.seats {
    //     println!("{:?}", v);
    // }
    // println!("--------------------------------------------------");
    // let state = state.updaten();
    // for v in &state.seats {
    //     println!("{:?}", v);
    // }
    // println!("--------------------------------------------------");
    // let state = state.updaten();
    // for v in &state.seats {
    //     println!("{:?}", v);
    // }
    // println!("--------------------------------------------------");
    //
    // return 0;
    rec(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
        return "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
    }

    // #[test]
    // fn test_gen() {
    //     let s = gen(get_input());
    //
    //     for v in s.seats {
    //         println!("{:?}", v);
    //     }
    //     println!("--------------------------------------------------");
    // }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&gen(get_input())), 37);
    }

    #[test]
    fn test_part2() {
//         assert_eq!(solve_part2(&gen(".......#.
// ...#.....
// .#.......
// .........
// ..#L....#
// ....#....
// .........
// #........
// ...#.....")), 26);
        assert_eq!(solve_part2(&gen(get_input())), 26);
    }
}