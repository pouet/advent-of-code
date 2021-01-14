use core::fmt;
use nom::lib::std::fmt::Formatter;

#[derive(Clone)]
pub struct Tile {
    num: usize,
    grid: Vec<Vec<char>>,
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Tile: {}\n", self.num)?;
        for v in &self.grid {
            for c in v {
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Tile {
    fn from_str(s: &str) -> Tile {
        let mut lines = s.lines();
        let num = lines.next().unwrap()[5..9].parse::<usize>().unwrap();
        let grid = lines.map(|s| s.chars().collect()).collect();

        Tile {
            num,
            grid,
        }
    }

    fn rotate(&mut self) {
        let grid = self.grid.clone();
        let len = grid.len();

        for i in 0..len {
            for j in 0..len {
                self.grid[len - i - 1][j] = grid[j][i];
            }
        }
    }

    fn flip(&mut self) {
        let grid = self.grid.clone();
        let len = grid.len();

        for i in 0..len {
            for j in 0..len {
                self.grid[len - j - 1][i] = grid[j][i];
            }
        }
    }

    fn get_top_edge(&self) -> Vec<char> {
        let mut v = Vec::new();
        let len = self.grid.len();

        for i in 0..len {
            v.push(self.grid[0][i]);
        }

        v
    }

    fn get_bottom_edge(&self) -> Vec<char> {
        let mut v = Vec::new();
        let len = self.grid.len();

        for i in 0..len {
            v.push(self.grid[len - 1][i]);
        }

        v
    }

    fn get_left_edge(&self) -> Vec<char> {
        let mut v = Vec::new();
        let len = self.grid.len();

        for i in 0..len {
            v.push(self.grid[i][0]);
        }

        v
    }


    fn get_right_edge(&self) -> Vec<char> {
        let mut v = Vec::new();
        let len = self.grid.len();

        for i in 0..len {
            v.push(self.grid[i][len - 1]);
        }

        v
    }

    fn get_edges(&self) -> Vec<Vec<char>> {
        vec![
            self.get_right_edge(),
            self.get_bottom_edge(),
            self.get_left_edge(),
            self.get_top_edge(),
            self.get_right_edge().into_iter().rev().collect(),
            self.get_bottom_edge().into_iter().rev().collect(),
            self.get_left_edge().into_iter().rev().collect(),
            self.get_top_edge().into_iter().rev().collect(),
        ]
    }

    fn is_adjacent(&self, tile: &Tile) -> bool {
        let edges = tile.get_edges();
        self
            .get_edges()
            .iter()
            .any(|e| edges
                .iter()
                .any(|f| e == f))
    }

    fn is_right(&self, tile: &Tile) -> bool {
        self.get_right_edge() == tile.get_left_edge()
    }

    // fn is_left(&self, tile: &Tile) -> bool {
    //     self.get_left_edge() == tile.get_right_edge()
    // }
    //
    // fn is_up(&self, tile: &Tile) -> bool {
    //     self.get_top_edge() == tile.get_bottom_edge()
    // }

    fn is_down(&self, tile: &Tile) -> bool {
        self.get_bottom_edge() == tile.get_top_edge()
    }

    fn edge_present(&self, edge: &Vec<char>, tiles: &Vec<Tile>) -> bool {
        tiles
            .iter()
            .filter(|t| t.num != self.num)
            .any(|t| {
                t.get_edges()
                    .iter()
                    .any(|e| *e == *edge)
            })
    }

    fn right_present(&self, tiles: &Vec<Tile>) -> bool {
        let edge = self.get_right_edge();
        self.edge_present(&edge, tiles)
    }

    // fn left_present(&self, tiles: &Vec<Tile>) -> bool {
    //     let edge = self.get_left_edge();
    //     self.edge_present(&edge, tiles)
    // }
    //
    // fn up_present(&self, tiles: &Vec<Tile>) -> bool {
    //     let edge = self.get_top_edge();
    //     self.edge_present(&edge, tiles)
    // }

    fn down_present(&self, tiles: &Vec<Tile>) -> bool {
        let edge = self.get_bottom_edge();
        self.edge_present(&edge, tiles)
    }

    fn count_sharp(&self) -> usize {
        self
            .grid
            .iter()
            .flatten()
            .filter(|&c| *c == '#')
            .count()
    }
}

fn count_adjacent(tile: &Tile, tiles: &Vec<Tile>) -> usize {
    tiles
        .iter()
        .filter(|&t| t.num != tile.num && tile.is_adjacent(t))
        .count()
}

fn get_top_left(tiles: &mut Vec<Tile>) -> Tile {
    let mut top_left = tiles
        .iter()
        .find(|&t| count_adjacent(t, &tiles) == 2)
        .unwrap()
        .clone();

    tiles.retain(|t| t.num != top_left.num);

    while !(top_left.right_present(&tiles) && top_left.down_present(&tiles)) {
        top_left.rotate();
    }

    top_left
}

#[derive(PartialEq)]
enum Dir {
    Right,
    Down
}

fn get_tile_from_dir(tile: &Tile, tiles: &mut Vec<Tile>, dir: Dir) -> Option<Tile> {
    let mut ret = None;

    'outer: for t in tiles.iter_mut() {
        for _ in 0..2 {
            for _ in 0..4 {
                if (dir == Dir::Right && tile.is_right(t)) ||
                    (dir == Dir::Down && tile.is_down(t)) {
                    ret = Some(t.clone());
                    break 'outer;
                }

                t.rotate();
            }
            t.flip();
        }
    }

    match ret {
        None => None,
        Some(t) => {
            tiles.retain(|t| t.num != tile.num);
            Some(t)
        }
    }
}

fn get_line(first: &Tile, tiles: &mut Vec<Tile>) -> Vec<Tile> {
    let mut line = vec![first.clone()];
    tiles.retain(|t| t.num != first.num);

    while let Some(tile) = get_tile_from_dir(&line.last().unwrap(), tiles, Dir::Right) {
        tiles.retain(|t| t.num != tile.num);
        line.push(tile.clone());
    }

    line
}

fn gen_tile_image(tiles: &Vec<Tile>) -> Vec<Vec<Tile>> {
    let mut tiles = tiles.clone();
    let mut current = get_top_left(&mut tiles);
    let mut grid: Vec<Vec<_>> = vec![];

    loop {
        let first = &current;
        let line = get_line(&current, &mut tiles);

        grid.push(line);

        current = match get_tile_from_dir(first, &mut tiles, Dir::Down) {
            None => break,
            Some(t) => t
        };
    }

    grid
}

fn gen_image_from_tiles(tiles: &Vec<Vec<Tile>>) -> Tile {
    let mut grid = vec![];

    for line in tiles.iter() {
        for i in 1..9 {
            let mut v = Vec::new();
            for col in line {
                for c in &col.grid[i][1..9] {
                    v.push(*c);
                }
            }
            grid.push(v);
        }
    }

    Tile {
        num: 0,
        grid,
    }
}

fn gen_image(tiles: &Vec<Tile>) -> Tile {
    let grid = gen_tile_image(tiles);
    gen_image_from_tiles(&grid)
}

fn count_monsters(tile: &Tile) -> Vec<(usize, usize)> {
    let coords = [
        (0, 18),
        (1, 0), (1, 5), (1, 6), (1, 11), (1, 12), (1, 17), (1, 18), (1, 19),
        (2, 1), (2, 4), (2, 7), (2, 10), (2, 13), (2, 16)
    ];
    let mut ret = vec![];

    for y in 0..tile.grid.len() - 2 {
        for x in 0..tile.grid[0].len() - 19 {
            if coords.iter().all(|(yy, xx)| tile.grid[y + yy][x + xx] == '#') {
                coords.iter().for_each(|(yy, xx)| ret.push((y + yy, x + xx)));
            }
        }
    }

    ret
}

fn find_monsters(tile: &Tile) -> Vec<(usize, usize)> {
    let mut tile = tile.clone();
    loop {
        for _ in 0..2 {
            for _ in 0..4 {
                let monsters = count_monsters(&tile);
                if !monsters.is_empty() {
                    return monsters;
                }
                tile.rotate();
            }
            tile.flip();
        }
    }
}

#[aoc_generator(day20)]
pub fn gen(input: &str) -> Vec<Tile> {
    input
        .split("\n\n")
        .map(Tile::from_str)
        .collect()
}

#[aoc(day20, part1)]
pub fn solve_part1(tiles: &Vec<Tile>) -> usize {
    tiles
        .iter()
        .filter(|&t| count_adjacent(t, tiles) == 2)
        .map(|t| t.num)
        .product()
}

#[aoc(day20, part2)]
pub fn solve_part2(tiles: &Vec<Tile>) -> usize {
    let puzzle = gen_image(&tiles);
    let monsters = find_monsters(&puzzle);

    puzzle.count_sharp() - monsters.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_gen() {
    //     println!("{:?}", gen(get_input()));
    // }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&gen(get_input())), 20899048083289);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&gen(get_input())), 273);
    }

    fn get_input() -> &'static str {
        return "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";
    }
}
