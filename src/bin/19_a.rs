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

#[derive(Debug, ParseDisplay, ParseFromStr)]
#[display("{no}: {rule}")]
struct RuleDesc {
    no: i32,
    rule: String,
}

fn rule_matches(rule: &Rule, rules: &HashMap<i32, Rule>, mut pattern: &[u8]) -> (bool, i32) {
    dbg!(&rule);
    dbg!(pattern);
    match rule {
        Rule::Literal(lit) => (!pattern.is_empty() && (pattern[0] == lit.lit as u8), 1),
        Rule::Or(or) => {
            if pattern.is_empty() {
                return (false, 0);
            }

            let (ok1, eaten1) = rule_matches(
                &Rule::Sequence(Sequence {
                    seq: vec![or.left_a, or.left_b],
                }),
                rules,
                pattern,
            );

            let (ok2, eaten2) = rule_matches(
                &Rule::Sequence(Sequence {
                    seq: vec![or.left_a, or.left_b],
                }),
                rules,
                pattern,
            );

            if ok1 {
                (ok1, eaten1)
            } else if ok2 {
                (ok2, eaten2)
            } else {
                (false, 0)
            }
        }
        Rule::Sequence(seq) => {
            let mut total_eaten = 0;

            for seq in seq.seq.iter() {
                if pattern.len() == 0 {
                    return (false, 0);
                }

                let (ok, eaten) = rule_matches(&rules[seq], rules, pattern);

                if !ok {
                    return (false, 0);
                }

                total_eaten += eaten;
                pattern = &pattern[(eaten as usize)..];
            }

            (true, total_eaten)
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

    patterns
        .lines()
        .filter(|p| rule_matches(&rules[&0], &rules, p.as_bytes()).0)
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
