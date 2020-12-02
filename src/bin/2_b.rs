use aoc2020::get_input;
use parse_display::{Display as ParseDisplay, FromStr as ParseFromStr};

#[derive(ParseDisplay, ParseFromStr)]
#[display("{first_pos}-{second_pos} {rule_letter}: {password}")]
struct CandidatePassword {
    first_pos: usize,
    second_pos: usize,
    rule_letter: char,
    password: String,
}

fn parse(line: &str) -> CandidatePassword {
    line.parse().unwrap()
}

fn password_valid(candidate: &CandidatePassword) -> bool {
    let bytes = candidate.password.as_bytes();
    (bytes[candidate.first_pos - 1] == candidate.rule_letter as u8)
        ^ (bytes[candidate.second_pos - 1] == candidate.rule_letter as u8)
}

fn main() {
    let valid_passwords = get_input()
        .lines()
        .map(parse)
        .filter(password_valid)
        .count();

    println!("{}", valid_passwords);
}
