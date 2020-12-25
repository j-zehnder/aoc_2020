use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub struct CPU {
    state: CpuState,
    ins_ptr: usize,
    instructions: Vec<Instruction>,
    acc_register: i64,
    visited_instructions: HashSet<usize>,
}

impl CPU {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        CPU {
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
            OpCode::ACC => {
                self.acc_register += instruction.val;
                self.ins_ptr += 1;
            }
            OpCode::JMP => self.ins_ptr = (self.ins_ptr as i64 + instruction.val) as usize,
            OpCode::NOP => self.ins_ptr += 1,
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

impl Instruction {
    pub fn from_str(s: &str) -> Self {
        let splits: Vec<&str> = s.split(" ").collect();
        let op = OpCode::from_str(splits.get(0).unwrap());
        let val = splits.get(1).unwrap().trim().parse::<i64>().unwrap();
        Instruction { op, val }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum OpCode {
    ACC,
    JMP,
    NOP,
}

impl OpCode {
    fn from_str(s: &str) -> Self {
        match s {
            "acc" => OpCode::ACC,
            "jmp" => OpCode::JMP,
            "nop" => OpCode::NOP,
            _ => panic!("invalid opcode"),
        }
    }
}

#[aoc_generator(day8)]
pub fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::from_str).collect()
}

#[aoc(day8, part1)]
pub fn part1(instructions: &[Instruction]) -> i64 {
    let mut cpu = CPU::new(instructions.to_owned());
    cpu.run();
    cpu.get_acc()
}

#[aoc(day8, part2)]
pub fn part2(instructions: &[Instruction]) -> i64 {
    for i in 0..instructions.len() {
        let mut new_instructions = instructions.to_owned();
        let ins_at = new_instructions.get(i).unwrap();
        if &ins_at.op == &OpCode::JMP {
            let new_instruction = Instruction {
                op: OpCode::NOP,
                val: ins_at.val,
            };
            new_instructions.remove(i);
            new_instructions.insert(i, new_instruction);
        } else if &ins_at.op == &OpCode::NOP {
            let new_instruction = Instruction {
                op: OpCode::JMP,
                val: ins_at.val,
            };
            new_instructions.remove(i);
            new_instructions.insert(i, new_instruction);
        }

        let mut new_cpu = CPU::new(new_instructions);
        let res = new_cpu.run();

        if res == &CpuState::Halted {
            return new_cpu.get_acc();
        }
    }
    panic!("answer not found");
}
