use aoc2020::{get_input, map::Map, pos::Pos};

fn find_adjacent_occupied(map: &Map, pos: Pos) -> i32 {
    let directions = &[
        Pos(-1, -1),
        Pos(0, -1),
        Pos(1, -1),
        Pos(-1, 0),
        Pos(1, 0),
        Pos(-1, 1),
        Pos(0, 1),
        Pos(1, 1),
    ];

    let mut adjacent_occupied = 0;

    for direction in directions {
        let mut search_pos = pos + *direction;

        loop {
            match map.get(search_pos) {
                None => break,
                Some('#') => {
                    adjacent_occupied += 1;
                    break;
                }
                Some('L') => break,
                _ => search_pos += *direction,
            }
        }
    }

    adjacent_occupied
}

fn apply_rules(map: &Map, pos: Pos, current: char) -> (Pos, char) {
    let adjacent_occupied = find_adjacent_occupied(map, pos);

    (
        pos,
        match current {
            'L' if adjacent_occupied == 0 => '#',
            '#' if adjacent_occupied >= 5 => 'L',
            _ => current,
        },
    )
}

fn next_map(map: &Map) -> Map {
    map.iter()
        .map(|(pos, current)| apply_rules(&map, pos, current))
        .collect::<Map>()
}

fn solve(input: &str) -> i32 {
    let mut map = Map::from_input(input);

    loop {
        let next = next_map(&map);

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

    use aoc2020::{map::Map, pos::Pos};

    #[test]
    fn find_adjacent_occupied() {
        assert_eq!(
            super::find_adjacent_occupied(
                &Map::from_input(
                    ".......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#....."
                ),
                Pos(3, 4)
            ),
            8
        );

        assert_eq!(
            super::find_adjacent_occupied(
                &Map::from_input(
                    ".............
.L.L.#.#.#.#.
............."
                ),
                Pos(1, 1)
            ),
            0
        );

        assert_eq!(
            super::find_adjacent_occupied(
                &Map::from_input(
                    ".##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##."
                ),
                Pos(3, 3)
            ),
            0
        );
    }

    #[test]
    fn test_next_map() {
        assert_eq!(
            super::next_map(&Map::from_input(
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
            )),
            Map::from_input(
                "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##"
            )
        );

        assert_eq!(
            super::next_map(&Map::from_input(
                "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##"
            )),
            Map::from_input(
                "#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#"
            )
        );
    }

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
            26
        );
    }
}
