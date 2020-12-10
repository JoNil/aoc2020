use aoc2020::get_input_i32;
use itertools::Itertools;
use std::iter;

fn find_adapter_chain(adapters: &[i32], current: i32, chain: &[i32]) -> Option<Vec<i32>> {
    if adapters.is_empty() {
        return Some(chain.into());
    }

    let imposible_next = adapters.iter().any(|a| *a < current);

    if imposible_next {
        return None;
    }

    let possible_next = adapters
        .iter()
        .filter(|a| *a - current <= 3)
        .copied()
        .collect_vec();

    for next in possible_next {
        let rest = adapters
            .iter()
            .filter(|c| **c != next)
            .copied()
            .collect_vec();

        let new_chain = chain.iter().chain(iter::once(&next)).copied().collect_vec();

        if let Some(chain) = find_adapter_chain(&rest, next, &new_chain) {
            return Some(chain);
        }
    }

    None
}

fn solve(adapters: &[i32]) -> (i32, i32) {
    let chain = find_adapter_chain(adapters, 0, &[]).unwrap();

    let mut last = 0;
    let mut dist_1_count = 0;
    let mut dist_3_count = 0;

    for val in &chain {
        match *val - last {
            1 => dist_1_count += 1,
            3 => dist_3_count += 1,
            _ => (),
        }
        last = *val;
    }

    (dist_1_count, dist_3_count + 1)
}

fn main() {
    let input = get_input_i32();

    let result = solve(&input);

    println!("{:?}", result.0 * result.1);
}

#[cfg(test)]
mod test {

    #[test]
    fn test_solve() {
        assert_eq!(
            super::solve(&[16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]),
            (7, 5)
        );

        assert_eq!(
            super::solve(&[
                28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25,
                35, 8, 17, 7, 9, 4, 2, 34, 10, 3
            ]),
            (22, 10)
        );
    }
}
