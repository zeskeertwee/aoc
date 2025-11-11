use std::clone::Clone;
use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::aoc_test;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Instruction {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

#[derive(Debug, Clone, Copy)]
enum ComboOperand {
    Value(u8),
    RegisterA,
    RegisterB,
    RegisterC,
    Invalid
}

impl From<u8> for Instruction {
    fn from(value: u8) -> Self {
        match value {
            0 => Instruction::Adv,
            1 => Instruction::Bxl,
            2 => Instruction::Bst,
            3 => Instruction::Jnz,
            4 => Instruction::Bxc,
            5 => Instruction::Out,
            6 => Instruction::Bdv,
            7 => Instruction::Cdv,
            _ => panic!("Invalid instruction {}", value)
        }
    }
}

impl From<u8> for ComboOperand {
    fn from(value: u8) -> Self {
        match value {
            0..4 => ComboOperand::Value(value),
            4 => ComboOperand::RegisterA,
            5 => ComboOperand::RegisterB,
            6 => ComboOperand::RegisterC,
            7 => ComboOperand::Invalid,
            _ => panic!("Invalid combo operand {}", value)
        }
    }
}

impl ComboOperand {
    fn as_literal(&self) -> u8 {
        match self {
            ComboOperand::Value(value) => *value,
            ComboOperand::RegisterA => 4,
            ComboOperand::RegisterB => 5,
            ComboOperand::RegisterC => 6,
            ComboOperand::Invalid => 7,
        }
    }
}

#[derive(Debug, Clone)]
struct Machine {
    registers: [usize; 3],
    instruction_ptr: usize,
    instructions: Vec<(Instruction, ComboOperand)>
}

impl Machine {
    fn get_combo_value(&self, combo_operand: &ComboOperand) -> usize {
        match combo_operand {
            ComboOperand::Value(v) => *v as usize,
            ComboOperand::RegisterA => self.registers[0],
            ComboOperand::RegisterB => self.registers[1],
            ComboOperand::RegisterC => self.registers[2],
            _ => panic!("Invalid combo operand {:?}", combo_operand)
        }
    }

    fn is_halted(&self) -> bool {
        self.instruction_ptr >= self.instructions.len()
    }

    fn step(&mut self) -> Option<u8> {
        let (inst, arg) = &self.instructions[self.instruction_ptr];
        if inst != &Instruction::Jnz {
            self.instruction_ptr += 1;
        }

        match inst {
            Instruction::Adv => self.registers[0] = self.registers[0] / (2usize.pow(self.get_combo_value(arg) as u32)),
            Instruction::Bxl => self.registers[1] = self.registers[1] ^ arg.as_literal() as usize,
            Instruction::Bst => self.registers[1] = self.get_combo_value(arg) % 8,
            Instruction::Jnz => {
                if self.registers[0] != 0 {
                    // instructions and arguments are merged, so 2 steps in the original is 1 now
                    self.instruction_ptr = arg.as_literal() as usize / 2;
                } else {
                    self.instruction_ptr = usize::MAX;
                }
            },
            Instruction::Bxc => self.registers[1] = self.registers[1] ^ self.registers[2],
            Instruction::Out => return Some((self.get_combo_value(arg) % 8) as u8),
            Instruction::Bdv => self.registers[1] = self.registers[0] / (2usize.pow(self.get_combo_value(arg) as u32)),
            Instruction::Cdv => self.registers[2] = self.registers[0] / (2usize.pow(self.get_combo_value(arg) as u32))
        }

        None
    }
}

#[aoc_generator(day17)]
fn parse_input(input: &str) -> (Machine, Vec<u8>) {
    let lines: Vec<&str> = input.lines().collect();
    let registers = [
        lines[0][12..].parse().unwrap(),
        lines[1][12..].parse().unwrap(),
        lines[2][12..].parse().unwrap()
    ];

    let numbers: Vec<u8> = lines[4][9..].trim().split(',').map(|v| v.parse::<u8>().unwrap()).collect();
    let instructions = numbers.as_chunks().0.into_iter().map(|[a, b]| {
        (Instruction::from(*a), ComboOperand::from(*b))
    }).collect();

    (Machine {
        registers, instructions, instruction_ptr: 0
    }, numbers)
}

#[aoc(day17, part1)]
fn part1(input: &(Machine, Vec<u8>)) -> String {
    let output = run_machine(input.0.clone()).into_iter().fold(String::new(), |mut acc, v| {
        acc.push_str(&v.to_string());
        acc.push(',');
        acc
    });

    output[..output.len() - 1].to_string()
}

#[aoc(day17, part2)]
fn part2(input: &(Machine, Vec<u8>)) -> usize {
    //let mut solution = 0usize;
    //dbg!(&input.1);
    //'outer: for i in input.1.iter().rev() {
    //    for a in 0..=0b111 {
    //        let mut machine = input.0.clone();
    //        machine.registers[0] = (solution << 3) + a;
    //        let out = run_machine(machine);
    //        println!("a = {}, i = {}, out = {}, reg = 0b{:b}", a, i, out[0], (solution << 3) + a);
    //        if out[0] == *i {
    //            dbg!(out[0], *i);
    //            number_solutions.push(solution);
    //            solution = (solution << 3) + a;
    //            continue 'outer;
    //        }
    //    }
    //
    //    // we need to backtrack to the previous number
    //    //panic!("Did not find solution!");
    //}
    search_solution(&mut input.1.clone(), input.0.clone(), 0).unwrap()
}

fn search_solution(target: &mut Vec<u8>, machine: Machine, solution: usize) -> Option<usize> {
    if target.is_empty() {
        return Some(solution);
    }

    let target_val = target.pop().unwrap();

    for a in 0..=0b111 {
        let mut m = machine.clone();
        m.registers[0] = (solution << 3) + a;
        let out = run_machine(m);
        
        if out[0] == target_val {
            match search_solution(target, machine.clone(), (solution << 3) + a) {
                Some(solution) => return Some(solution),
                None => continue,
            }
        }
    }

    target.push(target_val);
    None
}

fn run_machine(mut machine: Machine) -> Vec<u8> {
    let mut output = Vec::new();
    while !machine.is_halted() {
        if let Some(out) = machine.step() {
            output.push(out);
        }
    }

    output
}

aoc_test!(test_day17_sample1, "../samples/day17.txt", "4,6,3,5,6,3,5,2,1,0");
aoc_test!(test_day17_sample2, "../samples/day17-2.txt", "5,7,3,0", 117440);
aoc_test!(test_day17, "../input/2024/day17.txt", "6,0,6,3,0,2,3,1,6", 236539226447469);
