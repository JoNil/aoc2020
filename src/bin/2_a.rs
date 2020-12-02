use aoc2020::get_input;
use text_io::scan;

struct CandidatePassword {
    min_count: usize,
    max_count: usize,
    rule_letter: char,
    password: String,
}

fn parse(line: &str) -> CandidatePassword {
    let min_count;
    let max_count;
    let rule_letter;
    let password;

    scan!(line.bytes() => "{}-{} {}: {}", min_count, max_count, rule_letter, password);

    CandidatePassword {
        min_count,
        max_count,
        rule_letter,
        password,
    }
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
