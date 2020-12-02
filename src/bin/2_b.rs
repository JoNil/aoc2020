use aoc2020::get_input;
use text_io::scan;

struct CandidatePassword {
    first_pos: usize,
    second_pos: usize,
    rule_letter: char,
    password: String,
}

fn parse(line: &str) -> CandidatePassword {
    let (first_pos, second_pos, rule_letter, password);

    scan!(line.bytes() => "{}-{} {}: {}", first_pos, second_pos, rule_letter, password);

    CandidatePassword {
        first_pos,
        second_pos,
        rule_letter,
        password,
    }
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
