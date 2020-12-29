use aoc2020::get_input;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn solve(input: &str) -> String {
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

    let mut done = Vec::new();

    while !allergen_can_be.is_empty() {
        let mut done_ingridinet = Vec::new();

        for (allergen, can_be) in allergen_can_be.iter() {
            if can_be.len() == 1 {
                let ingridinet = *can_be.iter().next().unwrap();
                done.push((*allergen, ingridinet));
                done_ingridinet.push(ingridinet);
            }
        }

        for (allergen, _) in done.iter().copied() {
            allergen_can_be.remove(allergen);
        }

        for (_, can_be) in allergen_can_be.iter_mut() {
            for ingridinet in done_ingridinet.iter().copied() {
                can_be.remove(ingridinet);
            }
        }
    }

    done.sort_unstable_by(|a, b| a.0.cmp(b.0));

    done.iter().map(|d| d.1).join(",")
}

fn main() {
    let input = get_input();

    let result = solve(&input);

    println!("{}", result);
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
            "mxmxvkd,sqjhc,fvjkl".to_string()
        );
    }
}
