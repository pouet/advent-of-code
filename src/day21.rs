use nom::lib::std::collections::{HashMap, HashSet};
use itertools::Itertools;
use std::ops::Not;

#[derive(Debug)]
pub struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

impl Food {
    fn from_str(s: &str) -> Food {
        let mut sp = s
            .trim_end_matches(')')
            .split(" (contains ");
        let ingredients = sp
            .next()
            .unwrap()
            .split(" ")
            .map(String::from)
            .collect();
        let allergens = sp
            .next()
            .unwrap()
            .split(", ")
            .map(String::from)
            .collect();

        Food {
            ingredients,
            allergens,
        }
    }
}

fn get_correspondences(foods: &[Food]) -> HashMap<String, String> {
    // allergen with ingredient list
    let mut candidates: HashMap<&String, HashSet<String>> = HashMap::new();

    for food in foods.iter() {
        for allergen in food.allergens.iter() {
            let ingredients = candidates
                .entry(allergen)
                .or_insert_with(|| food.ingredients.clone());
            *ingredients = ingredients
                .intersection(&food.ingredients)
                .map(|s| s.clone())
                .collect();
        }
    }

    let mut res = HashMap::new();
    while let Some((i, a)) = candidates.iter().find(|(_, h)| h.len() == 1) {
        let a = a.iter().next().unwrap().clone();
        res.insert(i.clone(), a.clone());

        for (_, h) in candidates.iter_mut() {
            h.remove(&a);
        }
    }

    res
        .iter()
        .map(|(&k, v)| (k.clone(), v.clone()))
        .collect()
}

#[aoc_generator(day21)]
pub fn gen(input: &str) -> Vec<Food> {
    input
        .lines()
        .map(Food::from_str)
        .collect()
}

#[aoc(day21, part1)]
pub fn solve_part1(foods: &Vec<Food>) -> usize {
    let cor = get_correspondences(foods);

    foods
        .iter()
        .map(|food| food.ingredients.iter().collect_vec())
        .flatten()
        .filter(|ingredient| cor.values().any(|s| ingredient == &s).not())
        .count()
}

#[aoc(day21, part2)]
pub fn solve_part2(foods: &[Food]) -> String {
    get_correspondences(foods)
        .iter()
        .sorted()
        .map(|(_, v)| v)
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
        return "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
    }

    #[test]
    fn test_gen() {
        println!("{:?}", gen(get_input()));

        let foods = gen(get_input());
        let cor = get_correspondences(&foods);

        println!("{:?}", cor);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&gen(get_input())), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&gen(get_input())), "mxmxvkd,sqjhc,fvjkl");
    }
}
