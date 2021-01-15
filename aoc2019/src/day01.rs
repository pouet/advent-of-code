#[aoc_generator(day1)]
pub fn gen(input: &str) -> Vec<i32> {
    input
        .lines()
        .flat_map(|s| s.parse::<i32>())
        .collect()
}

fn fuel_from_mass(mass: &i32) -> i32 {
    (mass / 3) - 2
}

fn total_fuel(mass: &i32) -> i32 {
    let mut total = 0;
    let mut n = *mass;

    while n >= 9 {
        n = fuel_from_mass(&n);
        total += n;
    }

    total
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    input
        .iter()
        .map(fuel_from_mass)
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i32]) -> i32 {
    input
        .iter()
        .map(total_fuel)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel() {
        assert_eq!(fuel_from_mass(&12), 2);
        assert_eq!(fuel_from_mass(&14), 2);
        assert_eq!(fuel_from_mass(&1969), 654);
        assert_eq!(fuel_from_mass(&100756), 33583);
    }

    #[test]
    fn test_total_fuel() {
        assert_eq!(total_fuel(&14), 2);
        assert_eq!(total_fuel(&1969), 966);
        assert_eq!(total_fuel(&100756), 50346);
    }
}