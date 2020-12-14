use aoc2020::get_input;
use itertools::Itertools;
use parse_display::{Display as ParseDisplay, FromStr as ParseFromStr};
use std::collections::HashMap;

#[derive(ParseDisplay, ParseFromStr)]
#[display("mask = {value}")]
struct Mask {
    value: String,
}

#[derive(ParseDisplay, ParseFromStr)]
#[display("mem[{address}] = {value}")]
struct Memory {
    address: u64,
    value: u64,
}

#[derive(Debug)]
enum Instruction {
    SetMask(u64, u64, Vec<(u64, u64)>),
    SetMemory(u64, u64),
}

fn parse_program(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            if line.starts_with("mask") {
                let mask = line.parse::<Mask>().unwrap();

                let or_mask = u64::from_str_radix(&mask.value.replace('X', "0"), 2).unwrap();
                let and_mask = u64::from_str_radix(&mask.value.replace('X', "1"), 2).unwrap();

                let mut floating_mask = Vec::new();

                for index in mask.value.chars().enumerate().filter_map(|(index, ch)| {
                    if let 'X' = ch {
                        Some(index)
                    } else {
                        None
                    }
                }) {
                    let pos = 1 << (35 - index);
                    floating_mask.push((pos, 0));
                    floating_mask.push((pos, 1));
                }

                Instruction::SetMask(or_mask, and_mask, floating_mask)
            } else if line.starts_with("mem") {
                let mem = line.parse::<Memory>().unwrap();

                Instruction::SetMemory(mem.address, mem.value)
            } else {
                panic!("Invalid instruction")
            }
        })
        .collect()
}

#[derive(Default)]
struct Simulator {
    memory: HashMap<u64, u64>,
    or_mask: u64,
    and_mask: u64,
    floating: Vec<(u64, u64)>,
}

impl Simulator {
    fn run_program(&mut self, program: &[Instruction]) {
        for instruction in program {
            match instruction {
                Instruction::SetMask(or_mask, and_mask, floating) => {
                    self.or_mask = *or_mask;
                    self.and_mask = *and_mask;
                    self.floating = floating.clone();
                }
                Instruction::SetMemory(address, value) => {
                    let pre_floating = (*address | self.or_mask) & self.and_mask;

                    for bits in self.floating.iter().permutations(self.floating.len() / 2) {
                        let mut address = pre_floating;

                        for (bit, value) in bits {
                            if *value == 1 {
                                address |= bit;
                            } else {
                                address &= !bit;
                            }
                        }

                        self.memory
                            .insert(address, (value | self.or_mask) & self.and_mask);
                    }
                }
            }
        }
    }
}

fn solve(input: &str) -> u64 {
    let program = parse_program(input);

    let mut simulator = Simulator::default();

    simulator.run_program(&program);

    simulator.memory.values().sum::<u64>()
}

fn main() {
    let input = get_input();

    let result = solve(&input);

    println!("{:?}", result);
}

#[cfg(test)]
mod test {

    #[test]
    fn test_solve() {
        assert_eq!(
            super::solve(
                "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"
            ),
            208
        );
    }
}
