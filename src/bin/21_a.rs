use aoc2020::get_input;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn solve(input: &str) -> i32 {
    let recipes: Vec<(Vec<&str>, Vec<&str>)> = input
        .lines()
        .map(|line| {
            let (ingredients, allergens) = line.split(" (contains ").collect_tuple().unwrap();
            (
                ingredients.split(' ').collect_vec(),
                allergens
                    .trim_end_matches(')')
                    .split(',')
                    .map(|s| s.trim())
                    .collect_vec(),
            )
        })
        .collect_vec();

    let possible_ingredient = recipes
        .iter()
        .flat_map(|recipe| &recipe.0)
        .copied()
        .collect::<HashSet<&str>>();

    let mut allergen_can_be = recipes
        .iter()
        .flat_map(|recipe| &recipe.1)
        .copied()
        .map(|allergen| (allergen, possible_ingredient.clone()))
        .collect::<HashMap<&str, HashSet<&str>>>();

    for (ingredients, allergens) in &recipes {
        let ingredients = ingredients.iter().copied().collect::<HashSet<_>>();

        for allergen in allergens.iter().copied() {
            let must_be_one_of = allergen_can_be.get_mut(allergen).unwrap();

            *must_be_one_of = must_be_one_of.intersection(&ingredients).copied().collect();
        }
    }

    recipes
        .iter()
        .flat_map(|recipe| &recipe.0)
        .copied()
        .filter(|ingredient| {
            !allergen_can_be
                .values()
                .any(|candidate| candidate.contains(ingredient))
        })
        .count() as i32
}

fn main() {
    let input = get_input();

    let result = solve(&input);

    println!("{:?}", result);
}

#[cfg(test)]
mod test {

    #[test]
    fn test_solve() {
        assert_eq!(
            super::solve(
                "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"
            ),
            5
        );
    }
}
