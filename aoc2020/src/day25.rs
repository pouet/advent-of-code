#[aoc_generator(day25)]
pub fn gen(input: &str) -> (usize, usize) {
    let mut lines = input.lines();
    let n1 = lines.next().unwrap().parse().unwrap();
    let n2 = lines.next().unwrap().parse().unwrap();

    (n1, n2)
}

#[aoc(day25, part1)]
pub fn solve_part1((n1, n2): &(usize, usize)) -> usize {
    compute_encryption_key(*n1, *n2)
}

fn compute_loop(subject: usize, size: usize) -> usize {
    mod_exp::mod_exp(subject, size, 20201227)
}

fn get_loop_size(n: usize) -> usize {
    for i in 1.. {
        if compute_loop(7, i) == n {
            return i;
        }
    }

    unreachable!()
}

fn compute_encryption_key(n1: usize, n2: usize) -> usize {
    let loop_n1 = get_loop_size(n1);
    let loop_n2 = get_loop_size(n2);
    let key1 = compute_loop(n1, loop_n2);
    let key2 = compute_loop(n2, loop_n1);

    assert_eq!(key1, key2);

    key1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
        return "5764801
17807724";
    }

    #[test]
    fn test_gen() {
        println!("{:?}", gen(get_input()));
    }

    #[test]
    fn test() {
        compute_encryption_key(5764801, 17807724);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&gen(get_input())), 14897079);
    }
}