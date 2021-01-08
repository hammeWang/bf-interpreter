
use std::collections::HashMap;
use std::io::Read;
/// Eight commands of Brainf*ck
#[derive(Debug, Clone)]
pub enum OpCode {
    Forward, // >>
    Backward, // <<
    Increase, // +
    Decrease, // -
    Write, // .
    Read, // ,
    BeginLoop, // [
    EndLoop // ]
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Forward,
    Backward,
    Increase,
    Decrease,
    Write,
    Read,
    Loop(Vec<Instruction>)
}

pub fn parse_1(code: String)-> Vec<OpCode> {
    let chars = code.chars();
    chars.map(|c| match c {
        '>' => Some(OpCode::Forward),
        '<' => Some(OpCode::Backward),
        '+' => Some(OpCode::Increase),
        '-' => Some(OpCode::Decrease),
        '.' => Some(OpCode::Write),
        ',' => Some(OpCode::Read),
        '[' => Some(OpCode::BeginLoop),
        ']' => Some(OpCode::EndLoop),
        _ => None,
    }).filter(|c| c.is_some())
        .map(|c| c.unwrap())
        .collect::<Vec<OpCode>>()
}

pub fn parse_2(op_codes: Vec<OpCode>) -> Vec<Instruction> {
    let mut loop_depth = 0u32;
    let mut instructions: HashMap<u32, Vec<Instruction>> = HashMap::new();
    instructions.insert(0, vec![]);
    for op in op_codes {
        let instruction_vec = instructions.get_mut(&loop_depth).unwrap();
        match op {
            OpCode::Forward => instruction_vec.push(Instruction::Forward),
            OpCode::Backward => instruction_vec.push(Instruction::Backward),
            OpCode::Increase => instruction_vec.push(Instruction::Increase),
            OpCode::Decrease => instruction_vec.push(Instruction::Decrease),
            OpCode::Write => instruction_vec.push(Instruction::Write),
            OpCode::Read => instruction_vec.push(Instruction::Read),
            OpCode::BeginLoop => {
                loop_depth += 1;
                // initialize a new loop instructions
                instructions.insert(loop_depth, vec![]);
            },
            OpCode::EndLoop => {
                loop_depth -= 1;
                let loop_instructs_vec = instruction_vec.drain(..).collect::<Vec<_>>();
                let mut upper_instructs = instructions.get_mut(&loop_depth).unwrap();
                upper_instructs.push(Instruction::Loop(loop_instructs_vec));
            },
        }
    }

    instructions.entry(0u32).or_default().to_owned()
}


pub struct Executor {
    tape: [u8; 1024000],
    pointer: usize
}

impl Executor {
    pub fn new() -> Self {
        Executor {tape: [0; 1024000], pointer: 0}
    }

    pub fn run(&mut self, opcodes: &Vec<Instruction>) {
        for opcode in opcodes {
            match &*opcode {
                Instruction::Forward => self.pointer += 1,
                Instruction::Backward => self.pointer -= 1,
                Instruction::Increase => self.tape[self.pointer] += 1,
                Instruction::Decrease => self.tape[self.pointer] -= 1,
                Instruction::Write => print!("{}", std::str::from_utf8(&self.tape[self.pointer..])
                        .expect("can not parse the content.")),
                Instruction::Read =>  {
                    let mut output = [0; 1];
                    std::io::stdin().read_exact(&mut output).expect("fail to read");
                    self.tape[self.pointer] = output[0];
                },
                Instruction::Loop(vec) => {
                    while self.tape[self.pointer] != 0 {
                        self.run(&vec)
                    }
                }
            }
        }
    }
}










