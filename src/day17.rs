use nom::lib::std::collections::HashSet;
use std::ops::Not;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    x: isize,
    y: isize,
    z: isize,
    w: Option<isize>,
}

trait HashUtils {
    fn count_active_neighbors(&self, p: &Point) -> usize;
    fn next_cube_active(&self, p: &Point) -> bool;
    fn next_state(&self) -> Self;
}

impl HashUtils for HashSet<Point> {
    fn count_active_neighbors(&self, p: &Point) -> usize {
        let mut actives = 0;
        let w_range = if p.w.is_none() { 1..=1 } else { -1..=1 };

        for w in w_range {
            for x in -1..=1 {
                for y in -1..=1 {
                    for z in -1..=1 {
                        if (p.w.is_some() && w == 0 && x == 0 && y == 0 && z == 0 ||
                            (p.w.is_none() && x == 0 && y == 0 && z == 0)).not() {
                            let p = Point {
                                x: p.x + x,
                                y: p.y + y,
                                z: p.z + z,
                                w: p.w.map(|n| n + w),
                            };
                            if self.contains(&p) {
                                actives += 1;
                            }
                        }
                    }
                }
            }
        }

        actives
    }

    fn next_cube_active(&self, p: &Point) -> bool {
        let n = self.count_active_neighbors(p);
        let active = self.contains(p);

        if active && (n != 2 && n != 3) {
            false
        } else if active.not() && n == 3 {
            true
        } else {
            active
        }
    }

    fn next_state(&self) -> Self {
        let mut h: Self = HashSet::new();

        self
            .iter()
            .for_each(|p| {
                let w_range = if p.w.is_none() { 1..=1 } else { -1..=1 };
                for w in w_range {
                    for x in -1..=1 {
                        for y in -1..=1 {
                            for z in -1..=1 {
                                let p = Point {
                                    x: p.x + x,
                                    y: p.y + y,
                                    z: p.z + z,
                                    w: p.w.map(|n| n + w),
                                };
                                if h.contains(&p).not() && self.next_cube_active(&p) {
                                    h.insert(p);
                                }
                            }
                        }
                    }
                }
            });

        h
    }
}

#[aoc_generator(day17)]
pub fn gen(input: &str) -> HashSet<Point> {
    let mut h: HashSet<Point> = HashSet::new();

    input
        .lines()
        .enumerate()
        .for_each(|(y, s)| s
            .chars()
            .enumerate()
            .for_each(|(x, v)| {
                if v == '#' {
                    h.insert(Point {
                        x: x as isize,
                        y: y as isize,
                        z: 0,
                        w: None,
                    });
                }
            })
        );

    h
}

fn solve(h: &HashSet<Point>) -> usize {
    let mut h = h.next_state();
    for _ in 1..6 {
        h = h.next_state();
    }
    h.len()
}

#[aoc(day17, part1)]
pub fn solve_part1(h: &HashSet<Point>) -> usize {
    solve(h)
}

#[aoc(day17, part2)]
pub fn solve_part2(h: &HashSet<Point>) -> usize {
    let h: HashSet<Point> = h
        .into_iter()
        .map(|x| Point { w: Some(0), ..*x })
        .collect();
    solve(&h)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
        return ".#.
..#
###";
    }

    #[test]
    fn test_gen() {
        // println!("{:?}", gen(get_input()));
    }


    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&gen(get_input())), 112);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&gen(get_input())), 848);
    }
}
