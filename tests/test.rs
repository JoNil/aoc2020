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
aoc_test!(test_8_a => "1816");
aoc_test!(test_8_b => "1149");
aoc_test!(test_9_a => "50047984");
aoc_test!(test_9_b => "5407707");
aoc_test!(test_10_a => "2592");
aoc_test!(test_10_b => "198428693313536");
aoc_test!(test_11_a => "2470");
aoc_test!(test_11_b => "2259");
aoc_test!(test_12_a => "1956");
aoc_test!(test_12_b => "126797");
aoc_test!(test_13_a => "3246");
aoc_test!(test_14_a => "12408060320841");
aoc_test!(test_15_a => "468");
aoc_test!(test_15_b => "1801753");
aoc_test!(test_16_a => "25895");
aoc_test!(test_16_b => "5865723727753");
aoc_test!(test_17_a => "388");
aoc_test!(test_17_b => "2280");
aoc_test!(test_18_a => "3885386961962");
aoc_test!(test_18_b => "112899558798666");
aoc_test!(test_19_a => "162");
