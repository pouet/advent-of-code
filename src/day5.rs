use itertools::Itertools;

type Id = usize;

#[derive(Debug, PartialEq)]
pub struct Seat {
    row: usize,
    column: usize,
    id: Id,
}

impl Seat {
    fn lower_half(lower: usize, upper: usize) -> (usize, usize) {
        let mid = (upper - lower) / 2;
        (lower, upper - mid - 1)
    }

    fn upper_half(lower: usize, upper: usize) -> (usize, usize) {
        let mid = (upper - lower) / 2;
        (lower + mid + 1, upper)
    }

    fn get_row(s: &str) -> usize {
        assert_eq!(s.len(), 7);

        s
            .chars()
            .fold((0, 127), |(lower, upper), c|
                match c {
                    'F' => Seat::lower_half(lower, upper),
                    'B' => Seat::upper_half(lower, upper),
                    _ => panic!("Invalid input {}", c)
                })
            .0
    }

    fn get_column(s: &str) -> usize {
        assert_eq!(s.len(), 3);

        s
            .chars()
            .fold((0, 7), |(lower, upper), c|
                match c {
                    'L' => Seat::lower_half(lower, upper),
                    'R' => Seat::upper_half(lower, upper),
                    _ => panic!("Invalid input {}", c)
                })
            .1
    }

    pub fn from(s: &str) -> Seat {
        assert_eq!(s.len(), 10);

        let seat = Seat {
            row: Seat::get_row(&s[..7]),
            column: Seat::get_column(&s[7..]),
            id: 0
        };
        Seat { id: (seat.row * 8) + seat.column, ..seat}
    }
}

#[aoc_generator(day5)]
pub fn gen(input: &str) -> Vec<Seat> {
    input
        .lines()
        .map(|s| Seat::from(s))
        .sorted_by(|a, b| Ord::cmp(&a.id, &b.id))
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Vec<Seat>) -> Option<usize> {
    input
        .iter()
        .last()
        .map(|s| s.id)
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Vec<Seat>) -> Option<usize> {
    const LEFT: usize = 0;
    const RIGHT: usize = 1;
    input
        .iter()
        .as_slice()
        .windows(2)
        .find(|seats| (seats[LEFT].id + 1 == seats[RIGHT].id - 1))
        .map(|seat| seat[LEFT].id + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_half() {
        assert_eq!(Seat::lower_half(0, 127), (0, 63));
        assert_eq!(Seat::upper_half(0, 63), (32, 63));
        assert_eq!(Seat::lower_half(32, 63), (32, 47));
        assert_eq!(Seat::upper_half(32, 47), (40, 47));
        assert_eq!(Seat::upper_half(40, 47), (44, 47));
        assert_eq!(Seat::lower_half(44, 47), (44, 45));

        assert_eq!(Seat::upper_half(0, 7), (4, 7));
        assert_eq!(Seat::lower_half(4, 7), (4, 5));
    }

    #[test]
    fn test_row() {
        assert_eq!(Seat::get_row("FBFBBFF"), 44);
        assert_eq!(Seat::get_row("BFFFBBF"), 70);
        assert_eq!(Seat::get_row("FFFBBBF"), 14);
        assert_eq!(Seat::get_row("BBFFBBF"), 102);
    }

    #[test]
    fn test_column() {
        assert_eq!(Seat::get_column("RLR"), 5);
        assert_eq!(Seat::get_column("RRR"), 7);
        assert_eq!(Seat::get_column("RRR"), 7);
        assert_eq!(Seat::get_column("RLL"), 4);
    }

    #[test]
    fn seat_from() {
        assert_eq!(Seat::from("FBFBBFFRLR"), Seat { row: 44, column: 5, id: 357 });
        assert_eq!(Seat::from("BFFFBBFRRR"), Seat { row: 70, column: 7, id: 567 });
        assert_eq!(Seat::from("FFFBBBFRRR"), Seat { row: 14, column: 7, id: 119 });
        assert_eq!(Seat::from("BBFFBBFRLL"), Seat { row: 102, column: 4, id: 820 });
    }
}