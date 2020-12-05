use std::collections::HashSet;

use aoc2020::get_input;

fn resolve_bsp(bytes: &[u8], min: i32, max: i32) -> i32 {
    if bytes.is_empty() {
        max
    } else {
        let range = max - min + 1;

        match bytes.first().unwrap() {
            b'F' | b'L' => resolve_bsp(&bytes[1..], min, max - range / 2),
            b'B' | b'R' => resolve_bsp(&bytes[1..], min + range / 2, max),
            _ => panic!("Invalid input"),
        }
    }
}

fn get_position(input: &[u8]) -> (i32, i32) {
    (
        resolve_bsp(&input[..7], 0, 127),
        resolve_bsp(&input[7..], 0, 7),
    )
}

fn compute_seat_id((row, col): &(i32, i32)) -> i32 {
    row * 8 + col
}

fn main() {
    let input = get_input();

    let mut min_col = i32::MAX;
    let mut max_col = 0;

    let mut min_row = i32::MAX;
    let mut max_row = 0;

    for (row, col) in input.lines().map(|line| get_position(line.as_bytes())) {
        min_col = min_col.min(col);
        max_col = max_col.max(col);

        min_row = min_row.min(row);
        max_row = max_row.max(row);
    }

    let mut seats = HashSet::new();

    for row in (min_row + 1)..max_row {
        for col in min_col..max_col {
            seats.insert((row, col));
        }
    }

    for pos in input.lines().map(|line| get_position(line.as_bytes())) {
        seats.remove(&pos);
    }

    let my_seat = seats.iter().next().unwrap();

    let my_seat_id = compute_seat_id(my_seat);

    println!("{:?}", my_seat_id);
}
