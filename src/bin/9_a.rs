use aoc2020::get_input_i32;
use itertools::Itertools;

fn is_sum_of_any_two(preamble: &[i32], value: i32) -> bool {
    preamble
        .iter()
        .permutations(2)
        .any(|v| v[0] + v[1] == value)
}

fn find_first_not_sum_of_two_previous(input: &[i32], preamble_size: usize) -> i32 {
    for (index, value) in (&input[preamble_size..]).iter().enumerate() {
        if !is_sum_of_any_two(&input[(index)..(index + preamble_size)], *value) {
            return *value;
        }
    }

    panic!("Failed to find solution")
}

fn main() {
    let input = get_input_i32();

    let result = find_first_not_sum_of_two_previous(&input, 25);

    println!("{:?}", result);
}

#[cfg(test)]
mod test {

    use itertools::Itertools;

    #[test]
    fn test_is_sum_of_any_two() {
        assert_eq!(super::is_sum_of_any_two(&(1..=25).collect_vec(), 26), true);
        assert_eq!(super::is_sum_of_any_two(&(1..=25).collect_vec(), 49), true);
        assert_eq!(super::is_sum_of_any_two(&(1..=25).collect_vec(), 50), false);
        assert_eq!(
            super::is_sum_of_any_two(&(1..=25).collect_vec(), 100),
            false
        );
    }

    #[test]
    fn test_find_first_not_sum_of_two_previous() {
        assert_eq!(
            super::find_first_not_sum_of_two_previous(
                &[
                    35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
                    309, 576
                ],
                5
            ),
            127
        );
    }
}
