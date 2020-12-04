#[derive(Debug)]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq)]
pub struct Size {
    height: usize,
    width: usize,
}

#[derive(Debug, PartialEq)]
pub struct Grid {
    grid: Vec<Vec<char>>,
    size: Size,
}

pub struct Slope {
    right: usize,
    down: usize,
}

impl Grid {
    pub fn new(grid: Vec<Vec<char>>) -> Grid {
        let h = grid.len();
        let w = grid[0].len();
        Grid {
            grid,
            size: Size {
                height: h,
                width: w,
            },
        }
    }

    pub fn is_in_bounds(&self, p: &Point) -> bool {
        p.x < self.size.height
    }

    pub fn is_empty(&self, p: &Point) -> bool {
        self.grid[p.y][p.x] == '.'
    }

    pub fn is_tree(&self, p: &Point) -> bool {
        self.grid[p.y][p.x] == '#'
    }
}

impl Point {
    fn step_by(&mut self, slope: &Slope, size: &Size) {
        self.x = (self.x + slope.right) % size.width;
        self.y = self.y + slope.down;
    }
}

#[aoc_generator(day3)]
pub fn gen(input: &str) -> Grid {
    let grid = input
        .lines()
        .map(|s| s.trim().chars().collect())
        .collect();

    return Grid::new(grid);
}

fn count_trees(grid: &Grid, slope: &Slope) -> usize {
    let mut point = Point { x: 0, y: 0 };
    let mut trees = 0;

    while point.y < grid.size.height {
        if grid.is_tree(&point) {
            trees += 1;
        }
        point.step_by(&slope, &grid.size);
    }

    return trees;
}

#[aoc(day3, part1)]
pub fn solve_part1(grid: &Grid) -> usize {
    let slope = Slope { right: 3, down: 1 };
    return count_trees(grid, &slope);
}

#[aoc(day3, part2)]
pub fn solve_part2(grid: &Grid) -> usize {
    let slopes = vec![
        Slope { right: 1, down: 1 },
        Slope { right: 3, down: 1 },
        Slope { right: 5, down: 1 },
        Slope { right: 7, down: 1 },
        Slope { right: 1, down: 2 }
    ];
    slopes
        .iter()
        .map(|slope| count_trees(grid, &slope))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
        return "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
    }

    #[test]
    fn test_gen() {
        let v = vec![
            vec!['.', '.', '#', '#', '.', '.', '.', '.', '.', '.', '.'],
            vec!['#', '.', '.', '.', '#', '.', '.', '.', '#', '.', '.'],
            vec!['.', '#', '.', '.', '.', '.', '#', '.', '.', '#', '.'],
            vec!['.', '.', '#', '.', '#', '.', '.', '.', '#', '.', '#'],
            vec!['.', '#', '.', '.', '.', '#', '#', '.', '.', '#', '.'],
            vec!['.', '.', '#', '.', '#', '#', '.', '.', '.', '.', '.'],
            vec!['.', '#', '.', '#', '.', '#', '.', '.', '.', '.', '#'],
            vec!['.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
            vec!['#', '.', '#', '#', '.', '.', '.', '#', '.', '.', '.'],
            vec!['#', '.', '.', '.', '#', '#', '.', '.', '.', '.', '#'],
            vec!['.', '#', '.', '.', '#', '.', '.', '.', '#', '.', '#']
        ];
        let grid = Grid {
            grid: v,
            size: Size {
                height: 11,
                width: 11,
            },
        };
        assert_eq!(grid, gen(&get_input()));
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&gen(&get_input())), 7);
    }

    #[test]
    fn test_count_trees() {
        assert_eq!(count_trees(&gen(&get_input()), &Slope { right: 1, down: 1 }), 2);
        assert_eq!(count_trees(&gen(&get_input()), &Slope { right: 3, down: 1 }), 7);
        assert_eq!(count_trees(&gen(&get_input()), &Slope { right: 5, down: 1 }), 3);
        assert_eq!(count_trees(&gen(&get_input()), &Slope { right: 7, down: 1 }), 4);
        assert_eq!(count_trees(&gen(&get_input()), &Slope { right: 1, down: 2 }), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&gen(&get_input())), 336);
    }
}