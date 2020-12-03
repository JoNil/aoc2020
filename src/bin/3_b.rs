use std::collections::HashMap;

use aoc2020::get_input;

fn count_all_tree_collisions(input: &str) -> i64 {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut map = HashMap::new();
    let mut module_x = 0;

    for (y, line) in input.lines().enumerate() {
        module_x = line.len();
        for (x, ch) in line.chars().enumerate() {
            map.insert((x, y), ch);
        }
    }

    let mut res = 1;

    for slope in &slopes {
        let mut tree_collisions = 0;
        let mut current_pos = (0, 0);

        while let Some(location) = map.get(&(current_pos.0 % module_x, current_pos.1)) {
            if *location == '#' {
                tree_collisions += 1;
            }
            current_pos = (current_pos.0 + slope.0, current_pos.1 + slope.1);
        }

        res *= tree_collisions;
    }

    res
}

fn main() {
    let input = get_input();

    let tree_collisions = count_all_tree_collisions(&input);

    println!("{}", tree_collisions);
}

#[cfg(test)]
mod test {
    #[test]
    fn test_count_all_tree_collisions() {
        let input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

        assert_eq!(super::count_all_tree_collisions(input), 336);
    }
}
