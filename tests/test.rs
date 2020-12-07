use assert_cmd::Command;

macro_rules! aoc_test {
    ($name:ident => $value:literal) => {
        #[test]
        fn $name() {
            Command::cargo_bin(stringify!($name).trim_start_matches("test_"))
                .unwrap()
                .assert()
                .success()
                .stdout(concat!($value, "\n"));
        }
    };
}

aoc_test!(test_1_a => "793524");
aoc_test!(test_1_b => "61515678");
aoc_test!(test_2_a => "396");
aoc_test!(test_2_b => "428");
aoc_test!(test_3_a => "159");
aoc_test!(test_3_b => "6419669520");
aoc_test!(test_4_a => "233");
aoc_test!(test_4_b => "111");
aoc_test!(test_5_a => "894");
aoc_test!(test_5_b => "579");
aoc_test!(test_6_a => "6170");
aoc_test!(test_6_b => "2947");
aoc_test!(test_7_a => "248");
aoc_test!(test_7_b => "57281");
