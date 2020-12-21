use aoc2020::get_input;
use itertools::Itertools;
use parse_display::{Display as ParseDisplay, FromStr as ParseFromStr};
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
#[display("{left_a} {left_b} | {right_a} {right_b}")]
struct Or {
    left_a: i32,
    left_b: i32,
    right_a: i32,
    right_b: i32,
}

#[derive(Debug)]
enum Rule {
    Literal(Literal),
    Sequence(Sequence),
    Or(Or),
}

impl Rule {
    fn get_sequence(&self) -> &Sequence {
        match self {
            Rule::Sequence(seq) => seq,
            _ => panic!("Not a sequence"),
        }
    }
}

#[derive(Debug, ParseDisplay, ParseFromStr)]
#[display("{no}: {rule}")]
struct RuleDesc {
    no: i32,
    rule: String,
}

fn literal_matches(lit: &Literal, rules: &HashMap<i32, Rule>, mut pattern: &[u8]) -> bool {
    !pattern.is_empty() && (pattern[0] == lit.lit as u8)
}

fn or_matches(or: &Or, rules: &HashMap<i32, Rule>, mut pattern: &[u8]) -> bool {
    if pattern.is_empty() {
        return false;
    }

    let (ok1, eaten1) = sequence_matches(
        &Sequence {
            seq: vec![or.left_a, or.left_b],
        },
        rules,
        pattern,
    );

    let (ok2, eaten2) = sequence_matches(
        &Sequence {
            seq: vec![or.left_a, or.left_b],
        },
        rules,
        pattern,
    );

    [(ok1, eaten1), (ok2, eaten2)]
}

fn sequence_matches(seq: &Sequence, rules: &HashMap<i32, Rule>, mut pattern: &[u8]) -> (bool, i32) {
    let mut total_eaten = 0;

    for (index, seq_index) in seq.seq.iter().enumerate() {
        if pattern.is_empty() {
            return (false, 0);
        }

        match &rules[seq_index] {
            Rule::Sequence(seq) => {}
            Rule::Or(or) => {
                let [a, b] = or_matches(or, rules, pattern);

                if a.0 {
                    let reset_with_a = sequence_matches(
                        &Sequence {
                            seq: seq.seq[((*seq_index + 1) as usize)..].to_owned(),
                        },
                        rules,
                        &pattern[(a.1 as usize)..],
                    );

                    if reset_with_a.0 {
                        return (true, )
                    }
                }
            }
            Rule::Literal(lit) => {}
        }

        total_eaten += eaten;
        pattern = &pattern[(eaten as usize)..];
    }

    (true, total_eaten)
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

    patterns
        .lines()
        .filter(|p| sequence_matches(rules[&0].get_sequence(), &rules, p.as_bytes()).0)
        .count() as i32
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
