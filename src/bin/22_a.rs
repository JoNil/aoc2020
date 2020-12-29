use aoc2020::get_input;
use itertools::Itertools;
use std::{cmp::Ordering, collections::VecDeque};

fn solve(input: &str) -> i32 {
    let (player_1, player_2) = input.split("\n\n").collect_tuple().unwrap();

    let mut player_1 = player_1
        .trim_start_matches("Player 1:")
        .trim()
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<VecDeque<i32>>();
    let mut player_2 = player_2
        .trim_start_matches("Player 2:")
        .trim()
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<VecDeque<i32>>();

    while !player_1.is_empty() && !player_2.is_empty() {
        let player_1_card = player_1.pop_front().unwrap();
        let player_2_card = player_2.pop_front().unwrap();

        match player_1_card.cmp(&player_2_card) {
            Ordering::Less => {
                player_2.push_back(player_2_card);
                player_2.push_back(player_1_card);
            }
            Ordering::Greater => {
                player_1.push_back(player_1_card);
                player_1.push_back(player_2_card);
            }
            Ordering::Equal => panic!("No cards are equal"),
        }
    }

    let winner = if player_1.is_empty() {
        &player_2
    } else {
        &player_1
    };

    winner
        .iter()
        .rev()
        .enumerate()
        .map(|(index, value)| (index as i32 + 1) * *value)
        .sum()
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
                "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"
            ),
            306
        );
    }
}
