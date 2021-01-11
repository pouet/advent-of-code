use nom::lib::std::collections::{VecDeque, HashSet};

#[derive(PartialEq)]
pub enum Player {
    P1,
    P2,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Game {
    p1: VecDeque<usize>,
    p2: VecDeque<usize>,
}

impl Game {
    fn from_str(s: &str) -> Game {
        let decks: Vec<VecDeque<usize>> = s
            .split("\n\n")
            .map(|s| s
                .split('\n')
                .skip(1)
                .flat_map(str::parse::<usize>)
                .collect()
            )
            .collect();

        Game {
            p1: decks[0].clone(),
            p2: decks[1].clone(),
        }
    }

    fn is_finished(&self) -> bool {
        self.p1.is_empty() || self.p2.is_empty()
    }

    fn score(&self) -> Option<usize> {
        let deck = if self.p1.is_empty() {
            Some(&self.p2)
        } else if self.p2.is_empty() {
            Some(&self.p1)
        } else {
            None
        };

        deck
            .map(|deck| deck
                .iter()
                .rev()
                .enumerate()
                .map(|(i, n)| (i + 1) * n)
                .sum())
    }

    fn play_highest(&mut self) {
        if !self.is_finished() {
            let n1 = self.p1.pop_front().unwrap();
            let n2 = self.p2.pop_front().unwrap();

            if n1 > n2 {
                self.p1.push_back(n1);
                self.p1.push_back(n2);
            } else {
                self.p2.push_back(n2);
                self.p2.push_back(n1);
            }
        }
    }

    fn play_remaining(&mut self) {
        if !self.is_finished() {
            let n1 = self.p1.pop_front().unwrap();
            let n2 = self.p2.pop_front().unwrap();

            let mut game = self.clone();
            game.p1.resize(n1, 0);
            game.p2.resize(n2, 0);

            let player = game.play_part2();
            match player {
                Player::P1 => {
                    self.p1.push_back(n1);
                    self.p1.push_back(n2);
                }
                Player::P2 => {
                    self.p2.push_back(n2);
                    self.p2.push_back(n1);
                }
            }
        }
    }

    fn play_part2(&mut self) -> Player {
        let mut h: HashSet<Game> = HashSet::new();

        while !self.is_finished() {
            if h.contains(&self) {
                return Player::P1;
            }
            h.insert(self.clone());

            let n1 = *self.p1.front().unwrap();
            let n2 = *self.p2.front().unwrap();
            if self.p1.len() - 1 >= n1 && self.p2.len() - 1 >= n2 {
                self.play_remaining();
            } else {
                self.play_highest();
            }
        }

        if self.p1.is_empty() {
            Player::P2
        } else {
            Player::P1
        }
    }
}

#[aoc_generator(day22)]
pub fn gen(input: &str) -> Game {
    Game::from_str(input)
}

#[aoc(day22, part1)]
pub fn solve_part1(game: &Game) -> Option<usize> {
    let mut game = game.clone();

    while !game.is_finished() {
        game.play_highest();
    }

    game.score()
}

#[aoc(day22, part2)]
pub fn solve_part2(game: &Game) -> Option<usize> {
    let mut game = game.clone();

    game.play_part2();
    game.score()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
        return "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";
    }

    #[test]
    fn test_gen() {
        println!("{:?}", gen(get_input()));
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&gen(get_input())), Some(306));
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&gen(get_input())), Some(291));
    }
}

