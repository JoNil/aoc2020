use aoc2020::get_input_i32;
use itertools::Itertools;

fn count_adapter_chains(adapters: &[i32], current: i32, goal: i32) -> i64 {
    if current == goal {
        return 1;
    }

    let possible_next = adapters
        .iter()
        .filter(|a| *a - current <= 3 && **a > current)
        .copied()
        .collect_vec();

    let mut permutations = 0;
    for next in possible_next {
        let rest = adapters
            .iter()
            .filter(|c| **c != next)
            .copied()
            .collect_vec();

        permutations += count_adapter_chains(&rest, next, goal);
    }

    permutations
}

fn solve(adapters: Vec<i32>) -> i64 {
    let adapters = {
        let mut adapters = adapters;
        adapters.push(0);
        adapters.sort_unstable();
        adapters.push(adapters.last().unwrap() + 3);
        adapters
    };

    let mut sub_groups = Vec::new();

    let mut current_start = 0;
    let mut last = 0;
    for (index, value) in adapters.iter().enumerate() {
        if *value - last == 3 {
            sub_groups.push((
                adapters[current_start],
                &adapters[current_start..(index + 1)],
            ));
            current_start = index;
        }

        last = *value;
    }

    let mut result = 1;

    for (last, sub_group) in sub_groups {
        result *= count_adapter_chains(sub_group, last, sub_group[sub_group.len() - 1])
    }

    result
}

fn main() {
    let input = get_input_i32();

    let result = solve(input);

    println!("{:?}", result);
}

#[cfg(test)]
mod test {

    #[test]
    fn test_solve() {
        assert_eq!(super::solve(vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]), 8);

        assert_eq!(
            super::solve(vec![
                28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25,
                35, 8, 17, 7, 9, 4, 2, 34, 10, 3
            ]),
            19208
        );
    }
}
