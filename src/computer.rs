use std::borrow::Cow;

use itertools::Itertools;

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

#[derive(Clone, Debug)]
pub struct Computer<'a> {
    pc: i32,
    acc: i32,
    program: Cow<'a, [Instruction]>,
}

pub fn load_program(input: &str) -> Vec<Instruction> {
    let mut program = Vec::new();

    for line in input.lines() {
        let (instr, param) = line.trim().split(' ').tuples::<(_, _)>().next().unwrap();

        let param = param.parse().unwrap();

        program.push(match instr {
            "acc" => Instruction::Acc(param),
            "jmp" => Instruction::Jmp(param),
            "nop" => Instruction::Nop(param),
            _ => panic!("Unsupported instruction: {}", instr),
        });
    }

    program
}

impl<'a> Computer<'a> {
    pub fn new(input: &str) -> Computer<'a> {
        Computer {
            pc: 0,
            acc: 0,
            program: Cow::from(load_program(input)),
        }
    }

    pub fn from_program(program: &'a [Instruction]) -> Computer<'a> {
        Computer {
            pc: 0,
            acc: 0,
            program: Cow::from(program),
        }
    }

    #[inline]
    pub fn get_pc(&self) -> i32 {
        self.pc
    }

    #[inline]
    pub fn get_acc(&self) -> i32 {
        self.acc
    }

    #[inline]
    pub fn get_program(&self) -> &[Instruction] {
        &self.program
    }

    #[inline]
    pub fn step(&mut self) -> bool {
        match self.program[self.pc as usize] {
            Instruction::Acc(data) => {
                self.acc += data;
                self.pc += 1;
            }
            Instruction::Jmp(data) => {
                self.pc += data;
            }
            Instruction::Nop(_) => {
                self.pc += 1;
            }
        }

        self.pc >= self.program.len() as i32
    }
}
