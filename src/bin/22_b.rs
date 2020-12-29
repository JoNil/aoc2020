use aoc2020::get_input;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

#[derive(Copy, Clone, Debug)]
enum Winner {
    Player1,
    Player2,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct State {
    player_1: VecDeque<i32>,
    player_2: VecDeque<i32>,
}

fn play_inner(mut state: State, seen_states: &mut HashSet<State>) -> Winner {
    while !state.player_1.is_empty() && !state.player_2.is_empty() {
        if seen_states.contains(&state) {
            return Winner::Player1;
        }

        seen_states.insert(state.clone());

        let player_1_card = state.player_1.pop_front().unwrap();
        let player_2_card = state.player_2.pop_front().unwrap();

        let winner = if state.player_1.len() as i32 >= player_1_card
            && state.player_2.len() as i32 >= player_2_card
        {
            play_inner(state.clone(), seen_states)
        } else if player_1_card > player_2_card {
            Winner::Player1
        } else {
            Winner::Player2
        };

        match winner {
            Winner::Player2 => {
                state.player_2.push_back(player_2_card);
                state.player_2.push_back(player_1_card);
            }
            Winner::Player1 => {
                state.player_1.push_back(player_1_card);
                state.player_1.push_back(player_2_card);
            }
        }
    }

    if state.player_1.is_empty() {
        Winner::Player2
    } else {
        Winner::Player1
    }
}

fn play(mut state: State, seen_states: &mut HashSet<State>) -> VecDeque<i32> {
    while !state.player_1.is_empty() && !state.player_2.is_empty() {
        dbg!(&state);

        if seen_states.contains(&state) {
            return state.player_1;
        }

        seen_states.insert(state.clone());

        let player_1_card = state.player_1.pop_front().unwrap();
        let player_2_card = state.player_2.pop_front().unwrap();

        let winner = if state.player_1.len() as i32 >= player_1_card
            && state.player_2.len() as i32 >= player_2_card
        {
            play_inner(state.clone(), seen_states)
        } else if player_1_card > player_2_card {
            Winner::Player1
        } else {
            Winner::Player2
        };

        match winner {
            Winner::Player2 => {
                state.player_2.push_back(player_2_card);
                state.player_2.push_back(player_1_card);
            }
            Winner::Player1 => {
                state.player_1.push_back(player_1_card);
                state.player_1.push_back(player_2_card);
            }
        }
    }

    if state.player_1.is_empty() {
        state.player_2
    } else {
        state.player_1
    }
}

fn solve(input: &str) -> i32 {
    let (player_1, player_2) = input.split("\n\n").collect_tuple().unwrap();

    let player_1 = player_1
        .trim_start_matches("Player 1:")
        .trim()
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<VecDeque<i32>>();
    let player_2 = player_2
        .trim_start_matches("Player 2:")
        .trim()
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<VecDeque<i32>>();

    let mut seen_states = HashSet::new();

    let winner = play(State { player_1, player_2 }, &mut seen_states);

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
            291
        );

        assert_eq!(
            super::solve(
                "Player 1:
43
19

Player 2:
2
29
14"
            ),
            105
        );
    }
}
