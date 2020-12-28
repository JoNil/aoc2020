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
    Repeate(i32),
    Inner(i32, i32),
}

#[derive(Debug, ParseDisplay, ParseFromStr)]
#[display("{no}: {rule}")]
struct RuleDesc {
    no: i32,
    rule: String,
}

fn expand_to_regex(rule: &Rule, rules: &HashMap<i32, Rule>, variant: i32) -> String {
    match rule {
        Rule::Literal(lit) => lit.lit.to_string(),
        Rule::Sequence(seq) => seq
            .seq
            .iter()
            .map(|i| &rules[i])
            .map(|r| expand_to_regex(r, rules, variant))
            .collect(),
        Rule::Or(or) => {
            let left = or
                .left
                .trim()
                .split(' ')
                .map(|d| d.parse::<i32>().unwrap())
                .map(|r| expand_to_regex(&rules[&r], rules, variant))
                .collect::<String>();

            let right = or
                .right
                .trim()
                .split(' ')
                .map(|d| d.parse::<i32>().unwrap())
                .map(|r| expand_to_regex(&rules[&r], rules, variant))
                .collect::<String>();

            format!("({}|{})", left, right)
        }
        Rule::Repeate(rep) => format!("({})+", expand_to_regex(&rules[rep], rules, variant)),
        Rule::Inner(left, right) => format!(
            r#"{}{{{}}}{}{{{}}}"#,
            expand_to_regex(&rules[left], rules, variant),
            variant,
            expand_to_regex(&rules[right], rules, variant),
            variant,
        ),
    }
}

fn solve(input: &str) -> i32 {
    let (rules, patterns) = input.split("\n\n").tuples().next().unwrap();

    let mut rules = rules
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

    rules.insert(8, Rule::Repeate(42));
    rules.insert(11, Rule::Inner(42, 31));

    let regex = (1..10)
        .map(|i| expand_to_regex(&rules[&0], &rules, i))
        .map(|r| Regex::new(&format!("^{}$", r)).unwrap())
        .collect_vec();

    patterns
        .lines()
        .filter(|p| regex.iter().any(|r| r.is_match(p)))
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
                r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#
            ),
            12
        );
    }
}
