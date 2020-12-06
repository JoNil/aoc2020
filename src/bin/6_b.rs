use aoc2020::get_input;
use std::collections::HashSet;

fn get_common_answers(input: &str) -> i32 {
    let mut common_answers = HashSet::new();

    for (i, person) in input.lines().enumerate() {
        let answers = person.chars().collect::<HashSet<_>>();

        if i == 0 {
            common_answers = answers;
        } else {
            common_answers = common_answers.intersection(&answers).copied().collect();
        }
    }

    common_answers.len() as i32
}

fn main() {
    let input = get_input();

    let sum_of_unique_answers: i32 = input.split("\n\n").map(get_common_answers).sum();

    println!("{}", sum_of_unique_answers);
}
