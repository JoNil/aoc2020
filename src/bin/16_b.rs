use aoc2020::get_input;
use itertools::Itertools;

use parse_display::{Display as ParseDisplay, FromStr as ParseFromStr};

#[derive(Copy, Clone, Debug, ParseDisplay, ParseFromStr)]
#[display("{min}-{max}")]
struct Range {
    min: i32,
    max: i32,
}

impl Range {
    fn contains(&self, val: i32) -> bool {
        val >= self.min && val <= self.max
    }

    fn overlapps(&self, other: &Range) -> bool {
        self.contains(other.min) || self.contains(other.max)
    }

    fn merge(&self, other: &Range) -> Range {
        Range {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }
}

#[derive(Debug, ParseDisplay, ParseFromStr)]
#[display("{name}: {a} or {b}")]
struct Rule {
    name: String,
    a: Range,
    b: Range,
}

fn solve(input: &str) -> (Vec<String>, Vec<i32>) {
    let (rules, ticket, nearby) = input.split("\n\n").tuples().next().unwrap();

    let rules = rules
        .lines()
        .map(|line| line.parse::<Rule>().unwrap())
        .collect_vec();

    let ticket = ticket
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|num| num.parse::<i32>().unwrap())
        .collect_vec();

    let nearby = nearby
        .lines()
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let mut ranges = Vec::new();

    for rule in &rules {
        for rule in &[rule.a, rule.b] {
            let mut indicies_to_delete = Vec::new();
            let mut merged = Vec::new();

            for (index, range) in ranges.iter().enumerate() {
                if rule.overlapps(range) {
                    merged.push(*range);
                    indicies_to_delete.push(index);
                }
            }

            for index in indicies_to_delete.iter().rev() {
                ranges.remove(*index);
            }

            let merged_range = merged.iter().fold(*rule, |a, b| a.merge(b));

            ranges.push(merged_range);
        }
    }

    let tickets = nearby
        .iter()
        .filter(|ticket| {
            ticket
                .iter()
                .all(|value| ranges.iter().any(|range| range.contains(*value)))
        })
        .collect_vec();

    let field_count = tickets[0].len();

    let mut matches = Vec::new();
    matches.resize_with(field_count, Vec::new);

    for field in 0..field_count {
        for rule in &rules {
            if tickets
                .iter()
                .map(|t| t[field])
                .all(|v| rule.a.contains(v) || rule.b.contains(v))
            {
                matches[field].push((field, rule));
            }
        }
    }

    let mut done = Vec::new();
    done.resize_with(field_count, String::new);

    loop {
        let mut compleated_indicies = Vec::new();

        for (index, candidates) in matches.iter_mut().enumerate() {
            if candidates.len() == 1 {
                done[candidates[0].0] = candidates[0].1.name.clone();
                compleated_indicies.push(index)
            } else {
                *candidates = candidates
                    .iter()
                    .filter(|candidate| !done.iter().any(|done| candidate.1.name == *done))
                    .copied()
                    .collect_vec();
            }
        }

        for index in compleated_indicies.iter().rev() {
            matches.remove(*index);
        }

        if matches.is_empty() {
            break;
        }
    }

    (done, ticket)
}

fn main() {
    let input = get_input();

    let (names, ticket) = solve(&input);

    let mut result = 1;

    for (name, value) in names.iter().zip(ticket.iter()) {
        if name.starts_with("departure") {
            result *= *value as i64;
        }
    }

    println!("{:?}", result);
}

#[cfg(test)]
mod test {

    #[test]
    fn test_solve() {
        assert_eq!(
            super::solve(
                "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"
            )
            .0,
            vec![
                String::from("row"),
                String::from("class"),
                String::from("seat"),
            ]
        );
    }
}
