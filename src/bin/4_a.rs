use aoc2020::get_input;

fn get_valid_passport_count(input: &str) -> usize {
    const REQUIERED_FIELDS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    input
        .split("\n\n")
        .filter(|p| REQUIERED_FIELDS.iter().all(|field| p.contains(field)))
        .count()
}

fn main() {
    let input = get_input();

    let valid_passports = get_valid_passport_count(&input);

    println!("{}", valid_passports);
}

#[cfg(test)]
mod test {
    #[test]
    fn test_get_valid_passport_count() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

        assert_eq!(super::get_valid_passport_count(input), 2);
    }
}
