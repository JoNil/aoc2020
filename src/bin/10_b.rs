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

    match possible_next.len() {
        0 => 0,
        1 => {
            let rest = adapters
                .iter()
                .filter(|c| !possible_next.contains(c))
                .copied()
                .collect_vec();

            count_adapter_chains(&rest, *possible_next.iter().max().unwrap(), goal)
        }
        2 => {
            let rest = adapters
                .iter()
                .filter(|c| !possible_next.contains(c))
                .copied()
                .collect_vec();

            2 * count_adapter_chains(&rest, *possible_next.iter().max().unwrap(), goal)
        }
        3 => {
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
        _ => panic!(),
    }
}

fn solve(adapters: &[i32]) -> i64 {
    let goal = adapters.iter().max().unwrap();

    count_adapter_chains(adapters, 0, *goal)
}

fn main() {
    let input = get_input_i32();

    let result = solve(&input);

    println!("{:?}", result);
}

#[cfg(test)]
mod test {

    #[test]
    fn test_solve() {
        assert_eq!(super::solve(&[16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]), 8);

        assert_eq!(
            super::solve(&[
                28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25,
                35, 8, 17, 7, 9, 4, 2, 34, 10, 3
            ]),
            19208
        );
    }
}
