use aoc2020::get_input;

fn byr_rule(data: &str) -> bool {
    if let Ok(byr) = data.parse::<i32>() {
        byr >= 1920 && byr <= 2002 && data.len() == 4
    } else {
        false
    }
}

fn iyr_rule(data: &str) -> bool {
    if let Ok(iyr) = data.parse::<i32>() {
        iyr >= 2010 && iyr <= 2020 && data.len() == 4
    } else {
        false
    }
}

fn eyr_rule(data: &str) -> bool {
    if let Ok(eyr) = data.parse::<i32>() {
        eyr >= 2020 && eyr <= 2030 && data.len() == 4
    } else {
        false
    }
}

fn hgt_rule(data: &str) -> bool {
    if data.ends_with("cm") {
        if let Ok(hgt) = data.trim_end_matches("cm").parse::<i32>() {
            hgt >= 150 && hgt <= 193
        } else {
            false
        }
    } else if data.ends_with("in") {
        if let Ok(hgt) = data.trim_end_matches("in").parse::<i32>() {
            hgt >= 59 && hgt <= 76
        } else {
            false
        }
    } else {
        false
    }
}

fn hcl_rule(data: &str) -> bool {
    data.starts_with('#')
        && data.len() == 7
        && data
            .chars()
            .skip(1)
            .all(|c| c.is_ascii_digit() || (c.is_ascii_hexdigit() && c.is_ascii_lowercase()))
}

fn ecl_rule(data: &str) -> bool {
    const VALID_EYE_COLOT: &[&str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    VALID_EYE_COLOT.iter().any(|ecl| *ecl == data)
}

fn pid_rule(data: &str) -> bool {
    if data.parse::<i32>().is_ok() {
        data.len() == 9
    } else {
        false
    }
}

fn is_passport_valid(passport: &str) -> bool {
    const REQUIERED_FIELDS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let all_fields_present = REQUIERED_FIELDS
        .iter()
        .all(|field| passport.contains(field));

    let mut all_rules_ok = true;

    for (field, data) in passport.split_whitespace().map(|f| {
        let mut i = f.split(':');
        (i.next().unwrap(), i.next().unwrap())
    }) {
        all_rules_ok &= match field {
            "byr" => byr_rule(data),
            "iyr" => iyr_rule(data),
            "eyr" => eyr_rule(data),
            "hgt" => hgt_rule(data),
            "hcl" => hcl_rule(data),
            "ecl" => ecl_rule(data),
            "pid" => pid_rule(data),
            _ => true,
        };
    }

    all_fields_present && all_rules_ok
}

fn get_valid_passport_count(input: &str) -> usize {
    input.split("\n\n").filter(|p| is_passport_valid(p)).count()
}

fn main() {
    let input = get_input();

    let valid_passports = get_valid_passport_count(&input);

    println!("{}", valid_passports);
}

#[cfg(test)]
mod test {
    #[test]
    fn test_byr_rule() {
        let cases = [
            ("1919", false),
            ("1920", true),
            ("2002", true),
            ("2003", false),
        ];

        for (case, expected) in &cases {
            assert_eq!(super::byr_rule(case), *expected);
        }
    }

    #[test]
    fn test_iyr_rule() {
        let cases = [
            ("2009", false),
            ("2010", true),
            ("2020", true),
            ("2021", false),
        ];

        for (case, expected) in &cases {
            assert_eq!(super::iyr_rule(case), *expected);
        }
    }

    #[test]
    fn test_eyr_rule() {
        let cases = [
            ("2019", false),
            ("2020", true),
            ("2030", true),
            ("2031", false),
        ];

        for (case, expected) in &cases {
            assert_eq!(super::eyr_rule(case), *expected);
        }
    }

    #[test]
    fn test_hgt_rule() {
        let cases = [
            ("60in", true),
            ("190cm", true),
            ("190in", false),
            ("190", false),
        ];

        for (case, expected) in &cases {
            assert_eq!(super::hgt_rule(case), *expected);
        }
    }

    #[test]
    fn test_hcl_rule() {
        let cases = [("#123abc", true), ("#123abz", false), ("123abc", false)];

        for (case, expected) in &cases {
            assert_eq!(super::hcl_rule(case), *expected);
        }
    }

    #[test]
    fn test_ecl_rule() {
        let cases = [("brn", true), ("wat", false)];

        for (case, expected) in &cases {
            assert_eq!(super::ecl_rule(case), *expected);
        }
    }

    #[test]
    fn test_pid_rule() {
        let cases = [("000000001", true), ("0123456789", false)];

        for (case, expected) in &cases {
            assert_eq!(super::pid_rule(case), *expected);
        }
    }

    #[test]
    fn test_is_passport_valid() {
        let cases = [
            (
                "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
                false,
            ),
            (
                "iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946",
                false,
            ),
            (
                "hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
                false,
            ),
            (
                "hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007",
                false,
            ),
            (
                "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f",
                true,
            ),
            (
                "eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
                true,
            ),
            (
                "hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022",
                true,
            ),
            (
                "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
                true,
            ),
        ];

        for (case, expected) in &cases {
            assert_eq!(super::is_passport_valid(dbg!(case)), *expected);
        }
    }
}
