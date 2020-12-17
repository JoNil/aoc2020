use std::collections::HashMap;

fn get_next_no(
    last_no_was_first_time: bool,
    last_no_was_last_on_round: i32,
    index: i32,
    input: &[i32],
) -> i32 {
    if index - 1 < input.len() as i32 {
        input[index as usize - 1]
    } else if last_no_was_first_time {
        0
    } else {
        index - last_no_was_last_on_round - 1
    }
}

fn solve(input: &[i32], target: i32) -> i32 {
    let mut last_spoken_on_turn = HashMap::new();

    let mut last_no = 0;
    let mut last_no_was_first_time = false;
    let mut last_no_was_last_on_round = 0;

    for index in 1..=target {
        last_no = get_next_no(
            last_no_was_first_time,
            last_no_was_last_on_round,
            index,
            input,
        );
        last_spoken_on_turn
            .entry(last_no)
            .and_modify(|e| {
                last_no_was_last_on_round = *e;
                *e = index;
                last_no_was_first_time = false;
            })
            .or_insert_with(|| {
                last_no_was_first_time = true;
                index
            });
    }

    last_no
}

fn main() {
    let input = &[6, 19, 0, 5, 7, 13, 1];

    let result = solve(input, 2020);

    println!("{:?}", result);
}

#[cfg(test)]
mod test {

    #[test]
    fn test_solve() {
        assert_eq!(super::solve(&[0, 3, 6], 2020), 436);
        assert_eq!(super::solve(&[1, 3, 2], 2020), 1);
        assert_eq!(super::solve(&[2, 1, 3], 2020), 10);
        assert_eq!(super::solve(&[1, 2, 3], 2020), 27);
        assert_eq!(super::solve(&[2, 3, 1], 2020), 78);
        assert_eq!(super::solve(&[3, 2, 1], 2020), 438);
        assert_eq!(super::solve(&[3, 1, 2], 2020), 1836);
    }
}
