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

fn compute_seat_id((row, col): (i32, i32)) -> i32 {
    row * 8 + col
}

fn main() {
    let input = get_input();

    let max_seat_id = input
        .lines()
        .map(|line| get_position(line.as_bytes()))
        .map(compute_seat_id)
        .max()
        .unwrap();

    println!("{}", max_seat_id);
}

#[cfg(test)]
mod test {
    #[test]
    fn test_resolve_bsp() {
        assert_eq!(super::get_position(b"FBFBBFFRLR"), (44, 5));
        assert_eq!(super::get_position(b"BFFFBBFRRR"), (70, 7));
        assert_eq!(super::get_position(b"FFFBBBFRRR"), (14, 7));
        assert_eq!(super::get_position(b"BBFFBBFRLL"), (102, 4));
    }

    #[test]
    fn test_compute_seat_id() {
        assert_eq!(
            super::compute_seat_id(super::get_position(b"FBFBBFFRLR")),
            357
        );
        assert_eq!(
            super::compute_seat_id(super::get_position(b"BFFFBBFRRR")),
            567
        );
        assert_eq!(
            super::compute_seat_id(super::get_position(b"FFFBBBFRRR")),
            119
        );
        assert_eq!(
            super::compute_seat_id(super::get_position(b"BBFFBBFRLL")),
            820
        );
    }
}
