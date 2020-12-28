use aoc2020::{get_input, Map, Vec2};
use itertools::Itertools;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Copy, Clone, EnumIter, Debug, PartialEq)]
enum Transform {
    None,
    Rot90,
    Rot180,
    Rot270,
    Flipped,
    FlippedRot90,
    FlippedRot180,
    FlippedRot270,
}

impl Transform {
    fn apply(self, x: i32, y: i32) -> Vec2 {
        match self {
            Transform::None => Vec2(x, y),
            Transform::Rot90 => Vec2(9 - y, x),
            Transform::Rot180 => Vec2(9 - x, 9 - y),
            Transform::Rot270 => Vec2(y, 9 - x),
            Transform::Flipped => Vec2(9 - x, y),
            Transform::FlippedRot90 => Vec2(9 - y, 9 - x),
            Transform::FlippedRot180 => Vec2(x, 9 - y),
            Transform::FlippedRot270 => Vec2(y, x),
        }
    }
}

fn adjacent_pos(pos: Vec2) -> impl Iterator<Item = Vec2> {
    [Vec2(1, 0), Vec2(0, 1), Vec2(-1, 0), Vec2(0, -1)]
        .iter()
        .copied()
        .map(move |dir| pos + dir)
}

fn maps_allign(
    (pos1, map1, transform1): (Vec2, &Map, Transform),
    (pos2, map2, transform2): (Vec2, &Map, Transform),
) -> bool {
    assert!(map1.width == 10);
    assert!(map1.height == 10);
    assert!(map2.width == 10);
    assert!(map2.height == 10);

    let dx = pos2.0 - pos1.0;
    let dy = pos2.1 - pos1.1;

    match (dx, dy) {
        (1, 0) => {
            let map1_x = map1.width - 1;
            let map2_x = 0;

            // Map 2 is right
            (0..map1.height).all(|y| {
                map1.get(transform1.apply(map1_x, y)).unwrap()
                    == map2.get(transform2.apply(map2_x, y)).unwrap()
            })
        }
        (-1, 0) => {
            let map1_x = 0;
            let map2_x = map2.width - 1;

            // Map 2 is left
            (0..map1.height).all(|y| {
                map1.get(transform1.apply(map1_x, y)).unwrap()
                    == map2.get(transform2.apply(map2_x, y)).unwrap()
            })
        }
        (0, 1) => {
            let map1_y = map1.height - 1;
            let map2_y = 0;

            // Map 2 is down
            (0..map1.width).all(|x| {
                map1.get(transform1.apply(x, map1_y)).unwrap()
                    == map2.get(transform2.apply(x, map2_y)).unwrap()
            })
        }
        (0, -1) => {
            let map1_y = 0;
            let map2_y = map2.height - 1;

            // Map 2 is up
            (0..map1.width).all(|x| {
                map1.get(transform1.apply(x, map1_y)).unwrap()
                    == map2.get(transform2.apply(x, map2_y)).unwrap()
            })
        }
        _ => panic!("Maps are not adjacent"),
    }
}

fn solve(input: &str) -> i64 {
    let tiles = input
        .trim()
        .split("\n\n")
        .map(|tile| tile.split(":\n").collect_tuple().unwrap())
        .map(|(tile, map)| {
            (
                tile.trim_start_matches("Tile ").parse::<i32>().unwrap(),
                Map::from_input(&map.trim()),
            )
        })
        .collect_vec();

    let mut assembled = HashMap::new();
    let mut open_positions = Vec::new();

    let first = &tiles[0];
    let mut rest = tiles[1..].iter().map(|(id, map)| (*id, map)).collect_vec();
    let mut current_index = 0;

    assembled.insert(Vec2(0, 0), (first.0, &first.1, Transform::None));
    adjacent_pos(Vec2(0, 0)).for_each(|pos| open_positions.push(pos));

    'main: loop {
        let current = &rest[current_index];

        'every_position: for pos in &open_positions {
            for transform in Transform::iter() {
                let all_adjacent_matches = adjacent_pos(*pos).all(|other_pos| {
                    assembled
                        .get(&other_pos)
                        .map(|other| {
                            maps_allign(
                                (other_pos, &other.1, other.2),
                                (*pos, &current.1, transform),
                            )
                        })
                        .unwrap_or(true)
                });

                if all_adjacent_matches {
                    assembled.insert(*pos, (current.0, &current.1, transform));
                    for pos in adjacent_pos(*pos) {
                        if !open_positions.contains(&pos) && !assembled.contains_key(&pos) {
                            open_positions.push(pos)
                        }
                    }
                    rest.remove(current_index);
                    break 'every_position;
                }
            }
        }

        if rest.is_empty() {
            break 'main;
        }

        current_index = (current_index + 1) % rest.len()
    }

    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for pos in assembled.keys() {
        min_x = min_x.min(pos.0);
        max_x = max_x.max(pos.0);
        min_y = min_y.min(pos.1);
        max_y = max_y.max(pos.1);
    }

    assembled.get(&Vec2(min_x, min_y)).unwrap().0 as i64
        * assembled.get(&Vec2(max_x, min_y)).unwrap().0 as i64
        * assembled.get(&Vec2(min_x, max_y)).unwrap().0 as i64
        * assembled.get(&Vec2(max_x, max_y)).unwrap().0 as i64
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
                r#"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."#
            ),
            20899048083289
        );
    }
}
