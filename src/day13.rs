use itertools::Itertools;

// Chinese remainder theorem
// https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

#[derive(Debug)]
pub struct State {
    time: i64,
    ids: Vec<(i64, i64)>,
}

#[aoc_generator(day13)]
pub fn gen(input: &str) -> State {
    let mut sp = input.split("\n");
    let time = sp.next().unwrap().parse().unwrap();
    let ids = sp.next().unwrap()
        .split(",")
        .enumerate()
        .filter(|(_, s)| *s != "x")
        .map(|(i, b)| (i as i64, b.parse::<i64>().unwrap()))
        .collect();

    State { time, ids }
}

#[aoc(day13, part1)]
pub fn solve_part1(state: &State) -> i64 {
    assert!(state.ids.len() > 0);
    let (id, diff) = state.ids
        .iter()
        .map(|(_, id)| (id, id - (state.time % id)))
        .min_by(|&(m, _), &(n, _)| n.cmp(&m))
        .unwrap();

    println!("{:?}, {:?}", id, diff);
    return id * diff;
}

#[aoc(day13, part2)]
pub fn solve_part2(state: &State) -> Option<i64> {
    let residues = state.ids.iter().map(|(_, bus)| *bus).collect_vec();
    let modulii = state.ids.iter().map(|(i, bus)| bus - i).collect_vec();
    chinese_remainder(&modulii[..], &residues[..])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
        return "939
7,13,x,x,59,x,31,19";
    }

    #[test]
    fn test_gen() {
        println!("{:?}", gen(get_input()));
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&gen(get_input())), 295);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&gen("0\n17,x,13,19")), Some(3417));
        assert_eq!(solve_part2(&gen("0\n67,7,59,61")), Some(754018));
        assert_eq!(solve_part2(&gen("0\n67,x,7,59,61")), Some(779210));
        assert_eq!(solve_part2(&gen("0\n67,7,x,59,61")), Some(1261476));
        assert_eq!(solve_part2(&gen("0\n1789,37,47,1889")), Some(1202161486));

    }
}