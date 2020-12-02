use aoc2020::get_input;
use parse_display::{Display as ParseDisplay, FromStr as ParseFromStr};

#[derive(ParseDisplay, ParseFromStr)]
#[display("{min_count}-{max_count} {rule_letter}: {password}")]
struct CandidatePassword {
    min_count: usize,
    max_count: usize,
    rule_letter: char,
    password: String,
}

fn parse(line: &str) -> CandidatePassword {
    line.parse().unwrap()
}

fn password_valid(candidate: &CandidatePassword) -> bool {
    let candidate_count = candidate.password.matches(candidate.rule_letter).count();
    candidate_count >= candidate.min_count && candidate_count <= candidate.max_count
}

fn main() {
    let valid_passwords = get_input()
        .lines()
        .map(parse)
        .filter(password_valid)
        .count();

    println!("{}", valid_passwords);
}
