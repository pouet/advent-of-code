use itertools::Itertools;

pub fn is_increase(pass: &[u32]) -> bool {
    pass.iter()
        .tuple_windows()
        .all(|(a, b)| a <= b)
}

pub fn find_double(pass: &[u32]) -> bool {
    pass.iter()
        .tuple_windows()
        .any(|(a, b)| a == b)
}

pub fn is_correctly_grouped(pass: &[u32]) -> bool {
    pass.iter()
        .group_by(|it| *it)
        .into_iter()
        .map(|(_, group)| group.count())
        .any(|n| n == 2)
}

pub fn is_secure1(pass: &Vec<u32>) -> bool {
    is_increase(pass) && find_double(pass)
}

pub fn is_secure2(pass: &Vec<u32>) -> bool {
    is_secure1(pass) && is_correctly_grouped(pass)
}

pub fn gen(input: &str) -> Vec<u32> {
    input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(_: &[u8]) -> usize {
    (138241..674034)
        .map(|n| gen(&n.to_string()))
        .filter(is_secure1).count()
}

#[aoc(day4, part2)]
pub fn solve_part2(_: &[u8]) -> usize {
    (138241..674034)
        .map(|n| gen(&n.to_string()))
        .filter(is_secure2).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen() {
        println!("{:?}", gen("111111"));
        println!("{:?}", gen("223450"));
        println!("{:?}", gen("123789"));
        println!("{:?}", gen("122345"));
        println!("{:?}", gen("111123"));
        println!("{:?}", gen("135679"));
    }

    #[test]
    fn test_part1() {
        assert_eq!(is_secure1(&gen("111111")), true);
        assert_eq!(is_secure1(&gen("122345")), true);
        assert_eq!(is_secure1(&gen("111123")), true);
        assert_eq!(is_secure1(&gen("135679")), false);
        assert_eq!(is_secure1(&gen("223450")), false);
        assert_eq!(is_secure1(&gen("123789")), false);
    }

    #[test]
    fn test_part2() {
        assert_eq!(is_secure2(&gen("111111")), false);
        assert_eq!(is_secure2(&gen("122345")), true);
        assert_eq!(is_secure2(&gen("112233")), true);
        assert_eq!(is_secure2(&gen("123444")), false);
        assert_eq!(is_secure2(&gen("111122")), true);
    }
}