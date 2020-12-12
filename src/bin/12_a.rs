use aoc2020::{get_input, Vec2};

fn parse_instruction(line: &str) -> (char, i32) {
    let (ins, amount) = line.split_at(1);
    (ins.as_bytes()[0] as char, amount.parse().unwrap())
}

fn evaluate_rules(input: &str) -> Vec2 {
    let mut dir = Vec2(1, 0);
    let mut pos = Vec2(0, 0);

    for (ins, amount) in input.lines().map(parse_instruction) {
        match ins {
            'N' => pos.1 += amount,
            'S' => pos.1 -= amount,
            'E' => pos.0 += amount,
            'W' => pos.0 -= amount,
            'L' => match amount {
                90 => dir = Vec2(-dir.1, dir.0),
                180 => dir = Vec2(-dir.0, -dir.1),
                270 => dir = Vec2(dir.1, -dir.0),
                _ => panic!("Invalid rotation {}", amount),
            },
            'R' => match amount {
                90 => dir = Vec2(dir.1, -dir.0),
                180 => dir = Vec2(-dir.0, -dir.1),
                270 => dir = Vec2(-dir.1, dir.0),
                _ => panic!("Invalid rotation {}", amount),
            },
            'F' => pos += dir * amount,
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

    use crate::Vec2;

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
            25
        );
    }

    #[test]
    fn test_evaluate_rules() {
        assert_eq!(super::evaluate_rules("F10"), Vec2(10, 0));

        assert_eq!(super::evaluate_rules("R90\nF10"), Vec2(0, -10));
        assert_eq!(super::evaluate_rules("R180\nF10"), Vec2(-10, 0));
        assert_eq!(super::evaluate_rules("R270\nF10"), Vec2(0, 10));

        assert_eq!(super::evaluate_rules("L90\nF10"), Vec2(0, 10));
        assert_eq!(super::evaluate_rules("L180\nF10"), Vec2(-10, 0));
        assert_eq!(super::evaluate_rules("L270\nF10"), Vec2(0, -10));

        assert_eq!(super::evaluate_rules("L90\nR90\nF10"), Vec2(10, 0));
        assert_eq!(super::evaluate_rules("L90\nR180\nF10"), Vec2(0, -10));
        assert_eq!(super::evaluate_rules("L90\nR270\nF10"), Vec2(-10, 0));

        assert_eq!(super::evaluate_rules("L90\nL90\nF10"), Vec2(-10, 0));
        assert_eq!(super::evaluate_rules("L90\nL180\nF10"), Vec2(0, -10));
        assert_eq!(super::evaluate_rules("L90\nL270\nF10"), Vec2(10, 0));
    }
}
