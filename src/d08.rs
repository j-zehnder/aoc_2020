use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Cpu {
    state: CpuState,
    ins_ptr: usize,
    instructions: Vec<Instruction>,
    acc_register: i64,
    visited_instructions: HashSet<usize>,
}

impl Cpu {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Cpu {
            state: CpuState::Ready,
            ins_ptr: 0,
            instructions,
            acc_register: 0,
            visited_instructions: HashSet::new(),
        }
    }

    pub fn get_acc(&self) -> i64 {
        self.acc_register
    }

    pub fn step(&mut self) -> &CpuState {
        // out of instructions is normal termination
        if self.ins_ptr >= self.instructions.len() {
            self.state = CpuState::Halted;
            return &self.state;
        }

        let instruction = self.instructions.get(self.ins_ptr).unwrap();

        // error if you try to run the same instruction more than once
        if !self.visited_instructions.insert(self.ins_ptr) {
            self.state = CpuState::Error;
        }

        if self.state != CpuState::Ready {
            return &self.state;
        }

        match &instruction.op {
            OpCode::Acc => {
                self.acc_register += instruction.val;
                self.ins_ptr += 1;
            }
            OpCode::Jmp => self.ins_ptr = (self.ins_ptr as i64 + instruction.val) as usize,
            OpCode::Nop => self.ins_ptr += 1,
        }

        &self.state
    }

    pub fn run(&mut self) -> &CpuState {
        while self.state == CpuState::Ready {
            self.step();
        }
        &self.state
    }
}

#[derive(Debug, PartialEq)]
pub enum CpuState {
    Ready,
    Halted,
    Error,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Instruction {
    pub op: OpCode,
    pub val: i64,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits: Vec<&str> = s.split(' ').collect();
        let op = OpCode::from_str(splits.get(0).unwrap());
        let val = splits.get(1).unwrap().trim().parse::<i64>().unwrap();
        Ok(Instruction { op, val })
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum OpCode {
    Acc,
    Jmp,
    Nop,
}

impl OpCode {
    fn from_str(s: &str) -> Self {
        match s {
            "acc" => OpCode::Acc,
            "jmp" => OpCode::Jmp,
            "nop" => OpCode::Nop,
            _ => panic!("invalid opcode"),
        }
    }
}

#[aoc_generator(day8)]
pub fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(|x| x.parse::<Instruction>().unwrap()).collect()
}

#[aoc(day8, part1)]
pub fn part1(instructions: &[Instruction]) -> i64 {
    let mut cpu = Cpu::new(instructions.to_owned());
    cpu.run();
    cpu.get_acc()
}

#[aoc(day8, part2)]
pub fn part2(instructions: &[Instruction]) -> i64 {
    for i in 0..instructions.len() {
        let mut new_instructions = instructions.to_owned();
        let ins_at = new_instructions.get(i).unwrap();
        if ins_at.op == OpCode::Jmp {
            let new_instruction = Instruction {
                op: OpCode::Nop,
                val: ins_at.val,
            };
            new_instructions.remove(i);
            new_instructions.insert(i, new_instruction);
        } else if ins_at.op == OpCode::Nop {
            let new_instruction = Instruction {
                op: OpCode::Jmp,
                val: ins_at.val,
            };
            new_instructions.remove(i);
            new_instructions.insert(i, new_instruction);
        }

        let mut new_cpu = Cpu::new(new_instructions);
        let res = new_cpu.run();

        if res == &CpuState::Halted {
            return new_cpu.get_acc();
        }
    }
    panic!("answer not found");
}
