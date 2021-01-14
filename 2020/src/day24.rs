use nom::lib::std::collections::HashMap;
use itertools::Itertools;

type Tiles = HashMap<Point, Color>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
pub struct Tile {
    dirs: Vec<Point>
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Color {
    Black,
    White,
}

const DIRS: [Point; 6] = [
    Point { x: 2, y: 0 },
    Point { x: -2, y: 0 },
    Point { x: 1, y: 1 },
    Point { x: -1, y: 1 },
    Point { x: 1, y: -1 },
    Point { x: -1, y: -1 },
];


impl Color {
    fn flip(self) -> Color {
        if self == Color::Black {
            Color::White
        } else {
            Color::Black
        }
    }
}

impl Tile {
    fn from(s: &str) -> Tile {
        let mut s = s;
        let mut v = Vec::new();

        loop {
            if s.is_empty() {
                break;
            }
            let (t, p) = match &s[0..1] {
                "e" => (&s[1..], Point { x: 2, y: 0 }),
                "w" => (&s[1..], Point { x: -2, y: 0 }),
                _ => (&s[2..], match &s[0..2] {
                    "ne" => Point { x: 1, y: 1 },
                    "nw" => Point { x: -1, y: 1 },
                    "se" => Point { x: 1, y: -1 },
                    "sw" => Point { x: -1, y: -1 },
                    _ => panic!("Invalid input")
                })
            };

            v.push(p);
            s = t;
        }

        Tile {
            dirs: v
        }
    }

    fn identify(&self) -> Point {
        self
            .dirs
            .iter()
            .fold(Point { x: 0, y: 0 }, |acc, p|
                Point { x: acc.x + p.x, y: acc.y + p.y })
    }
}


fn flip_all(tiles: &Vec<Tile>) -> Tiles {
    let mut h = HashMap::new();

    tiles
        .iter()
        .map(Tile::identify)
        .for_each(|p| {
            let col = h.entry(p).or_insert(Color::White);
            *col = col.flip();
        });

    h
}

fn count_black_neighbors(tiles: &Tiles, p: &Point) -> usize {
    DIRS
        .iter()
        .filter(|&d| {
            let x = Point { x: p.x + d.x, y: p.y + d.y };
            matches!(tiles.get(&x), Some(col) if *col == Color::Black)
        })
        .count()
}

fn get_points(tiles: &Tiles) -> Vec<Point> {
    tiles
        .keys()
        .flat_map(|p| DIRS
            .iter()
            .map(|d| Point { x: p.x + d.x, y: p.y + d.y })
            .collect_vec()
        )
        .unique()
        .collect()
}

fn day(tiles: &Tiles) -> Tiles {
    get_points(&tiles)
        .iter()
        .filter(|&p| {
            let n = count_black_neighbors(&tiles, p);
            match tiles.get(p) {
                Some(&Color::Black) => n == 1 || n == 2,
                _ => n == 2,
            }
        })
        .map(|&p| (p, Color::Black))
        .collect()
}

#[aoc_generator(day24)]
pub fn gen(input: &str) -> Vec<Tile> {
    input
        .lines()
        .map(Tile::from)
        .collect()
}

#[aoc(day24, part1)]
pub fn solve_part1(tiles: &Vec<Tile>) -> usize {
    flip_all(tiles)
        .values()
        .filter(|&&c| c == Color::Black)
        .count()
}

#[aoc(day24, part2)]
pub fn solve_part2(tiles: &Vec<Tile>) -> usize {
    let tiles = flip_all(tiles);

    (0..100)
        .fold(tiles, |tiles, _| day(&tiles))
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
        return "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
    }

    #[test]
    fn test_gen() {
        // println!("{:?}", gen(get_input()));
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&gen(get_input())), 10);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&gen(get_input())), 2208);
    }
}