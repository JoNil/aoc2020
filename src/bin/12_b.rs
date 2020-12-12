use aoc2020::{get_input, Vec2};

fn parse_instruction(line: &str) -> (char, i32) {
    let (ins, amount) = line.split_at(1);
    (ins.as_bytes()[0] as char, amount.parse().unwrap())
}

fn evaluate_rules(input: &str) -> Vec2 {
    let mut wp = Vec2(10, 1);
    let mut pos = Vec2(0, 0);

    for (ins, amount) in input.lines().map(parse_instruction) {
        match ins {
            'N' => wp.1 += amount,
            'S' => wp.1 -= amount,
            'E' => wp.0 += amount,
            'W' => wp.0 -= amount,
            'L' => match amount {
                90 => wp = Vec2(-wp.1, wp.0),
                180 => wp = Vec2(-wp.0, -wp.1),
                270 => wp = Vec2(wp.1, -wp.0),
                _ => panic!("Invalid rotation {}", amount),
            },
            'R' => match amount {
                90 => wp = Vec2(wp.1, -wp.0),
                180 => wp = Vec2(-wp.0, -wp.1),
                270 => wp = Vec2(-wp.1, wp.0),
                _ => panic!("Invalid rotation {}", amount),
            },
            'F' => pos += wp * amount,
            _ => panic!("Invalid instruction {}", ins),
        }
    }

    pos
}

fn solve(input: &str) -> i32 {
    let pos = evaluate_rules(input);
    pos.0.abs() + pos.1.abs()
}

fn main() {
    let input = get_input();

    let result = solve(&input);

    println!("{:?}", result);
}

#[cfg(test)]
mod test {

    #[test]
    fn test_solve() {
        assert_eq!(
            super::solve(
                "F10
N3
F7
R90
F11"
            ),
            286
        );
    }
}
