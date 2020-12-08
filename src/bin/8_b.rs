use aoc2020::{
    computer::{load_program, Computer, Instruction},
    get_input,
};
use std::collections::HashSet;

fn run_to_end_or_inf_loop(computer: &mut Computer) -> Option<i32> {
    let mut visited_pc = HashSet::new();

    while !visited_pc.contains(&computer.get_pc()) {
        visited_pc.insert(computer.get_pc());
        if computer.step() {
            return Some(computer.get_acc());
        }
    }

    None
}

fn swap_nop_jmp(instr: &Instruction) -> Instruction {
    match instr {
        Instruction::Nop(data) => Instruction::Jmp(*data),
        Instruction::Jmp(data) => Instruction::Nop(*data),
        Instruction::Acc(data) => Instruction::Acc(*data),
    }
}

fn solve(input: &str) -> i32 {
    let mut program = load_program(input);

    for index in 0..program.len() {
        program[index] = swap_nop_jmp(&program[index]);

        if let Some(result) = run_to_end_or_inf_loop(&mut Computer::from_program(&program)) {
            return result;
        }

        program[index] = swap_nop_jmp(&program[index]);
    }

    panic!("Didn't find solution")
}

fn main() {
    let input = get_input();

    let solution = solve(&input);

    println!("{:?}", solution);
}

#[cfg(test)]
mod test {

    #[test]
    fn test_solve() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

        assert_eq!(super::solve(input), 8);
    }
}
