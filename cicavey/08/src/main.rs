use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

#[derive(Debug, Clone)]
struct DecodeError {
    details: String,
}

impl DecodeError {
    fn new(msg: String) -> DecodeError {
        DecodeError { details: msg }
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for DecodeError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Debug, Clone, PartialEq)]
enum OpCode {
    NOP,
    ACC,
    JMP,
}

#[derive(Debug, Clone)]
struct Ins(OpCode, i32);

fn decode(code: String) -> Result<Ins, DecodeError> {
    let s: Vec<&str> = code.split_ascii_whitespace().collect();
    let opcode = *s.get(0).unwrap();
    match opcode {
        "nop" => {
            let arg1 = s.get(1).unwrap().parse::<i32>();
            Ok(Ins(OpCode::NOP, arg1.unwrap()))
        }
        "acc" => {
            let arg1 = s.get(1).unwrap().parse::<i32>();
            Ok(Ins(OpCode::ACC, arg1.unwrap()))
        }
        "jmp" => {
            let arg1 = s.get(1).unwrap().parse::<i32>();
            Ok(Ins(OpCode::JMP, arg1.unwrap()))
        }
        _ => Err(DecodeError::new(format!("Unknown opcode: {}", opcode))),
    }
}

fn run(program: Vec<Ins>) -> (i64, bool) {
    let mut pc: i32 = 0;
    let mut ac: i64 = 0;
    let mut v = HashSet::new();

    loop {
        if v.contains(&pc) {
            return (ac, false);
        }

        if pc >= program.len() as i32 {
            return (ac, true);
        }

        // fetch
        let ins = program.get(pc as usize).unwrap();
        v.insert(pc);

        // exec
        match ins.0 {
            OpCode::NOP => pc += 1,
            OpCode::JMP => pc += ins.1,
            OpCode::ACC => {
                ac += ins.1 as i64;
                pc += 1
            }
        }
    }
}

fn main() {
    let mut program = Vec::new();
    for line in lines_from_file("input.txt") {
        program.push(decode(line).unwrap())
    }

    // part 1
    dbg!(run(program.clone()));

    // part 2 - brute force by trying to change each instruction

    // For each original instruction, clone program, replace, and try again
    for (i, ins) in program.iter().enumerate() {
        let mut new_prog = program.clone();

        // Swap instructions
        if ins.0 == OpCode::JMP {
            let mut new_ins = ins.clone();
            new_ins.0 = OpCode::NOP;
            new_prog[i] = new_ins;
        } else if ins.0 == OpCode::NOP {
            let mut new_ins = ins.clone();
            new_ins.0 = OpCode::NOP;
            new_prog[i] = new_ins;
        }

        let (ac, finished) = run(new_prog);

        if finished {
            dbg!(ac);
            break;
        }
    }
}
