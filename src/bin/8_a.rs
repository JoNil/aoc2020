use aoc2020::{computer::Computer, get_input};
use std::collections::HashSet;

fn find_accumulator_before_inf_loop(input: &str) -> i32 {
    let mut computer = Computer::new(&input);
    let mut visited_pc = HashSet::new();

    while !visited_pc.contains(&computer.get_pc()) {
        visited_pc.insert(computer.get_pc());
        computer.step();
    }

    computer.get_acc()
}

fn main() {
    let input = get_input();

    let accumulator_before_inf_loop = find_accumulator_before_inf_loop(&input);

    println!("{:?}", accumulator_before_inf_loop);
}

#[cfg(test)]
mod test {

    #[test]
    fn test_find_accumulator_before_inf_loop() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

        assert_eq!(super::find_accumulator_before_inf_loop(input), 5);
    }
}
