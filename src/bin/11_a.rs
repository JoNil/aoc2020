use aoc2020::{get_input, map::Map, pos::Pos};

fn apply_rules(map: &Map, pos: Pos, current: char) -> (Pos, char) {
    let offsets = &[
        Pos(-1, -1),
        Pos(0, -1),
        Pos(1, -1),
        Pos(-1, 0),
        Pos(1, 0),
        Pos(-1, 1),
        Pos(0, 1),
        Pos(1, 1),
    ];

    let adjacent_occupied = offsets
        .iter()
        .filter_map(|p| map.get(pos + *p))
        .filter(|c| *c == '#')
        .count();

    (
        pos,
        match current {
            'L' if adjacent_occupied == 0 => '#',
            '#' if adjacent_occupied >= 4 => 'L',
            _ => current,
        },
    )
}

fn solve(input: &str) -> i32 {
    let mut map = Map::from_input(input);

    loop {
        let next = map
            .iter()
            .map(|(pos, current)| apply_rules(&map, pos, current))
            .collect::<Map>();

        if map == next {
            break;
        }

        map = next;
    }

    map.values().filter(|c| *c == '#').count() as i32
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
                "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"
            ),
            37
        );
    }
}
