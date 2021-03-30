use fancy_regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Instruction {
    Mask(String),
    Mem(usize, usize),
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        let re_mask = Regex::new(r"^mask = (.{36})$").unwrap();
        let re_mem = Regex::new(r"^mem\[(\d*)] = (\d*)$").unwrap();

        if re_mask.is_match(s).unwrap() {
            let captures = re_mask.captures(s).unwrap().unwrap();
            let mask = captures.get(1).unwrap().as_str().to_string();
            return Instruction::Mask(mask);
        } else if re_mem.is_match(s).unwrap() {
            let captures = re_mem.captures(s).unwrap().unwrap();
            let index = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let val = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
            return Instruction::Mem(index, val);
        }
        panic!("invalid format")
    }
}

struct Cpu {
    mask: String,
    mem: HashMap<usize, usize>,
}

impl Cpu {
    fn new() -> Self {
        Cpu {
            mask: "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_string(),
            mem: HashMap::new(),
        }
    }

    fn process(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Mask(mask) => self.mask = mask.clone(),
            Instruction::Mem(i, v) => {
                // apply mask
                let vstr = format!("{:036b}", v);

                let mut val_string = String::new();
                self.mask
                    .chars()
                    .into_iter()
                    .zip(vstr.chars().into_iter())
                    .for_each(|i| match i.0 {
                        'X' => val_string = format!("{}{}", val_string, i.1),
                        '1' => val_string = format!("{}{}", val_string, 1),
                        '0' => val_string = format!("{}{}", val_string, 0),
                        _ => panic!("invalid mask"),
                    });
                self.mem
                    .insert(*i, usize::from_str_radix(val_string.as_str(), 2).unwrap());
            }
        }
    }

    fn process2(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Mask(mask) => self.mask = mask.clone(),
            Instruction::Mem(mem, value) => {
                for mem_address in apply_mask(self.mask.clone(), *mem) {
                    self.mem.insert(mem_address, *value);
                }
            }
        }
    }
}

fn apply_mask(mask: String, mem: usize) -> Vec<usize> {
    let memstr = format!("{:036b}", mem);

    let mut addresses: Vec<String> = vec!["".to_string()];

    mask.chars()
        .into_iter()
        .zip(memstr.chars().into_iter())
        .for_each(|i| match i.0 {
            '0' => {
                let mut new_addresses: Vec<String> = Vec::new();
                for a in &addresses {
                    let new_a = format!("{}{}", a, i.1);
                    new_addresses.push(new_a);
                }
                addresses = new_addresses;
            }
            '1' => {
                let mut new_addresses: Vec<String> = Vec::new();
                for a in &addresses {
                    let new_a = format!("{}{}", a, 1);
                    new_addresses.push(new_a);
                }
                addresses = new_addresses;
            }
            'X' => {
                let mut new_addresses: Vec<String> = Vec::new();
                for a in &addresses {
                    let new_a_0 = format!("{}{}", a, 0);
                    let new_a_1 = format!("{}{}", a, 1);
                    new_addresses.push(new_a_0);
                    new_addresses.push(new_a_1);
                }
                addresses = new_addresses;
            }
            _ => panic!("unknown mask"),
        });

    let mut memory_addresses: Vec<usize> = Vec::new();
    for mem in addresses {
        memory_addresses.push(usize::from_str_radix(mem.as_str(), 2).unwrap());
    }
    memory_addresses
}

#[aoc_generator(day14)]
pub fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::from_str).collect()
}

#[aoc(day14, part1)]
pub fn part1(instructions: &[Instruction]) -> usize {
    let mut cpu = Cpu::new();
    instructions.iter().for_each(|i| cpu.process(i));
    cpu.mem.values().sum()
}

#[aoc(day14, part2)]
pub fn part2(instructions: &[Instruction]) -> usize {
    let mut cpu = Cpu::new();
    instructions.iter().for_each(|i| cpu.process2(i));
    cpu.mem.values().sum()
}
