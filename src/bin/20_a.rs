use std::collections::HashMap;

use aoc2020::{get_input, Map, Vec2};
use itertools::Itertools;

fn adjacent_pos(pos: Vec2) -> impl Iterator<Item = Vec2> {
    [Vec2(1, 0), Vec2(0, 1), Vec2(-1, 0), Vec2(0, -1)]
        .iter()
        .copied()
        .map(move |dir| pos + dir)
}

fn solve(input: &str) -> i64 {
    let tiles = input
        .split("\n\n")
        .map(|tile| tile.split(":\n").collect_tuple().unwrap())
        .map(|(tile, map)| {
            (
                tile.trim_start_matches("Tile ").parse::<i32>().unwrap(),
                Map::from_input(&map.trim()),
            )
        })
        .collect_vec();

    dbg!(&tiles);

    let mut assembled = HashMap::new();
    let mut open_positions = Vec::new();

    let first = &tiles[0];
    let rest = &tiles[1..];

    assembled.insert(Vec2(0, 0), first);
    for pos in adjacent_pos(Vec2(0, 0)) {
        open_positions.push(pos);
    }

    0
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
