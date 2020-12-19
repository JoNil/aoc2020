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

fn solve(input: &str) -> i32 {
    let (rules, ticket, nearby) = input.split("\n\n").tuples().next().unwrap();

    let rules = rules
        .lines()
        .map(|line| line.parse::<Rule>().unwrap())
        .collect_vec();

    let _ticket = ticket
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

    let mut ticket_scanning_error_rate = 0;

    for ticket in &nearby {
        for value in ticket {
            if !ranges.iter().any(|range| range.contains(*value)) {
                ticket_scanning_error_rate += value;
            }
        }
    }

    ticket_scanning_error_rate
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
                "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"
            ),
            71
        );
    }
}
