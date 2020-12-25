use aoc2020::get_input;
use itertools::Itertools;
use parse_display::{Display as ParseDisplay, FromStr as ParseFromStr};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, ParseDisplay, ParseFromStr)]
#[display("\"{lit}\"")]
struct Literal {
    lit: char,
}

#[derive(Debug)]
struct Sequence {
    seq: Vec<i32>,
}

#[derive(Debug, ParseDisplay, ParseFromStr)]
#[display("{left} | {right}")]
struct Or {
    left: String,
    right: String,
}

#[derive(Debug)]
enum Rule {
    Literal(Literal),
    Sequence(Sequence),
    Or(Or),
}

#[derive(Debug, ParseDisplay, ParseFromStr)]
#[display("{no}: {rule}")]
struct RuleDesc {
    no: i32,
    rule: String,
}

fn expand_to_regex(rule: &Rule, rules: &HashMap<i32, Rule>) -> String {
    match rule {
        Rule::Literal(lit) => lit.lit.to_string(),
        Rule::Sequence(seq) => seq
            .seq
            .iter()
            .map(|i| &rules[i])
            .map(|r| expand_to_regex(r, rules))
            .collect(),
        Rule::Or(or) => {
            let left = or
                .left
                .trim()
                .split(' ')
                .map(|d| d.parse::<i32>().unwrap())
                .map(|r| expand_to_regex(&rules[&r], rules))
                .collect::<String>();

            let right = or
                .right
                .trim()
                .split(' ')
                .map(|d| d.parse::<i32>().unwrap())
                .map(|r| expand_to_regex(&rules[&r], rules))
                .collect::<String>();

            format!("({}|{})", left, right)
        }
    }
}

fn solve(input: &str) -> i32 {
    let (rules, patterns) = input.split("\n\n").tuples().next().unwrap();

    let rules = rules
        .lines()
        .map(|line| {
            let desc = line.parse::<RuleDesc>().unwrap();
            let rule = desc.rule.trim();

            if let Ok(lit) = rule.parse::<Literal>() {
                return (desc.no, Rule::Literal(lit));
            }

            if let Ok(or) = rule.parse::<Or>() {
                return (desc.no, Rule::Or(or));
            }

            (
                desc.no,
                Rule::Sequence(Sequence {
                    seq: rule
                        .split(' ')
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect_vec(),
                }),
            )
        })
        .collect::<HashMap<_, _>>();

    let regex = expand_to_regex(&rules[&0], &rules);
    let regex = Regex::new(&format!("^{}$", regex)).unwrap();

    patterns.lines().filter(|p| regex.is_match(p)).count() as i32
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
                r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#
            ),
            2
        );
    }
}
