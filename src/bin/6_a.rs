use aoc2020::get_input;
use itertools::Itertools;

fn get_unique_answers(input: &str) -> i32 {
    input
        .chars()
        .filter(|c| !c.is_whitespace())
        .unique()
        .count() as i32
}

fn main() {
    let input = get_input();

    let sum_of_unique_answers: i32 = input.split("\n\n").map(get_unique_answers).sum();

    println!("{}", sum_of_unique_answers);
}
