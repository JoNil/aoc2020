use aoc2020::get_input_i32;
use itertools::Itertools;

fn is_sum_of_any_two(preamble: &[i32], value: i32) -> bool {
    preamble
        .iter()
        .permutations(2)
        .any(|v| v[0] + v[1] == value)
}

fn find_first_not_sum_of_two_previous(input: &[i32], preamble_size: usize) -> (usize, i32) {
    for (index, value) in (&input[preamble_size..]).iter().enumerate() {
        if !is_sum_of_any_two(&input[(index)..(index + preamble_size)], *value) {
            return (index, *value);
        }
    }

    panic!("Failed to find solution")
}

fn find_weakness(input: &[i32], preamble_size: usize) -> i32 {
    let (anomaly_index, anomaly) = find_first_not_sum_of_two_previous(input, preamble_size);

    for sum_window_start in 0..anomaly_index {
        for sum_window_end in sum_window_start..anomaly_index {
            if input[sum_window_start..sum_window_end]
                .iter()
                .copied()
                .sum::<i32>()
                == anomaly
            {
                let max = input[sum_window_start..sum_window_end]
                    .iter()
                    .max()
                    .unwrap();
                let min = input[sum_window_start..sum_window_end]
                    .iter()
                    .min()
                    .unwrap();

                return min + max;
            }
        }
    }

    panic!("Failed to find weekness");
}

fn main() {
    let input = get_input_i32();

    let result = find_weakness(&input, 25);

    println!("{:?}", result);
}

#[cfg(test)]
mod test {
    #[test]
    fn test_find_weakness() {
        assert_eq!(
            super::find_weakness(
                &[
                    35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
                    309, 576
                ],
                5
            ),
            62
        );
    }
}
