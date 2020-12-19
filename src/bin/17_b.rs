use aoc2020::get_input;
use std::collections::HashSet;

static NEIGHBORS: [(i32, i32, i32, i32); 80] = [
    (-1, -1, -1, -1),
    (0, -1, -1, -1),
    (1, -1, -1, -1),
    (-1, 0, -1, -1),
    (0, 0, -1, -1),
    (1, 0, -1, -1),
    (-1, 1, -1, -1),
    (0, 1, -1, -1),
    (1, 1, -1, -1),
    (-1, -1, 0, -1),
    (0, -1, 0, -1),
    (1, -1, 0, -1),
    (-1, 0, 0, -1),
    (0, 0, 0, -1),
    (1, 0, 0, -1),
    (-1, 1, 0, -1),
    (0, 1, 0, -1),
    (1, 1, 0, -1),
    (-1, -1, 1, -1),
    (0, -1, 1, -1),
    (1, -1, 1, -1),
    (-1, 0, 1, -1),
    (0, 0, 1, -1),
    (1, 0, 1, -1),
    (-1, 1, 1, -1),
    (0, 1, 1, -1),
    (1, 1, 1, -1),
    (-1, -1, -1, 0),
    (0, -1, -1, 0),
    (1, -1, -1, 0),
    (-1, 0, -1, 0),
    (0, 0, -1, 0),
    (1, 0, -1, 0),
    (-1, 1, -1, 0),
    (0, 1, -1, 0),
    (1, 1, -1, 0),
    (-1, -1, 0, 0),
    (0, -1, 0, 0),
    (1, -1, 0, 0),
    (-1, 0, 0, 0),
    (1, 0, 0, 0),
    (-1, 1, 0, 0),
    (0, 1, 0, 0),
    (1, 1, 0, 0),
    (-1, -1, 1, 0),
    (0, -1, 1, 0),
    (1, -1, 1, 0),
    (-1, 0, 1, 0),
    (0, 0, 1, 0),
    (1, 0, 1, 0),
    (-1, 1, 1, 0),
    (0, 1, 1, 0),
    (1, 1, 1, 0),
    (-1, -1, -1, 1),
    (0, -1, -1, 1),
    (1, -1, -1, 1),
    (-1, 0, -1, 1),
    (0, 0, -1, 1),
    (1, 0, -1, 1),
    (-1, 1, -1, 1),
    (0, 1, -1, 1),
    (1, 1, -1, 1),
    (-1, -1, 0, 1),
    (0, -1, 0, 1),
    (1, -1, 0, 1),
    (-1, 0, 0, 1),
    (0, 0, 0, 1),
    (1, 0, 0, 1),
    (-1, 1, 0, 1),
    (0, 1, 0, 1),
    (1, 1, 0, 1),
    (-1, -1, 1, 1),
    (0, -1, 1, 1),
    (1, -1, 1, 1),
    (-1, 0, 1, 1),
    (0, 0, 1, 1),
    (1, 0, 1, 1),
    (-1, 1, 1, 1),
    (0, 1, 1, 1),
    (1, 1, 1, 1),
];

fn get_neighbors((x, y, z, w): (i32, i32, i32, i32)) -> impl Iterator<Item = (i32, i32, i32, i32)> {
    NEIGHBORS
        .iter()
        .map(move |&(dx, dy, dz, dw)| (x + dx, y + dy, z + dz, w + dw))
}

fn solve(input: &str) -> i32 {
    let mut active = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                active.insert((x as i32, y as i32, 0, 0));
            }
        }
    }

    for _step in 0..6 {
        let mut new = HashSet::new();

        for &coord in &active {
            let active_neighbors = get_neighbors(coord)
                .filter(|coord| active.contains(coord))
                .count();

            if active_neighbors == 2 || active_neighbors == 3 {
                new.insert(coord);
            }
        }

        let candidates = active
            .iter()
            .flat_map(|&coord| get_neighbors(coord))
            .filter(|coord| !active.contains(&coord));

        for candidate in candidates {
            let active_neighbors = get_neighbors(candidate)
                .filter(|coord| active.contains(coord))
                .count();

            if active_neighbors == 3 {
                new.insert(candidate);
            }
        }

        active = new;
    }

    active.len() as i32
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
                ".#.
..#
###"
            ),
            848
        );
    }
}
