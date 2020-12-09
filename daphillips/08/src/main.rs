use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

#[derive(Clone, PartialEq)]
enum OpType {
    ACC,
    JMP,
    NOP,
}

impl FromStr for OpType {
    type Err = ();

    fn from_str(input: &str) -> Result<OpType, Self::Err> {
        match input.to_uppercase().as_str() {
            "ACC" => Ok(OpType::ACC),
            "JMP" => Ok(OpType::JMP),
            "NOP" => Ok(OpType::NOP),
            _ => Err(()),
        }
    }
}

#[derive(Clone)]
struct Instruction {
    op: OpType,
    arg: i32,
}

#[derive(Clone)]
struct Interpreter {
    instructions: Vec<Instruction>, // I want this to be backed by a mutable vec (like kotlin)
    pc: usize,                      // TODO should this ever be negative?
    accumulator: i32,
}

impl Interpreter {
    fn new(lines: &Vec<String>) -> Interpreter {
        let mut instructions = Vec::new();
        for line in lines {
            let parsed_line: Vec<&str> = line.split(' ').collect();
            let op = OpType::from_str(parsed_line.get(0).unwrap()).unwrap();
            let arg = parsed_line.get(1).unwrap().parse().unwrap();
            instructions.push(Instruction { arg, op })
        }
        Interpreter {
            instructions,
            pc: 0,
            accumulator: 0,
        }
    }

    fn execute_step(&mut self) {
        // TODO we should be prepared to handle going out of bounds of the instructions!
        let instruction: &Instruction = self.instructions.get(self.pc).unwrap();
        match instruction.op {
            OpType::ACC => {
                self.accumulator = self.accumulator + instruction.arg;
                self.pc += 1;
            }
            // convert to signed for the math then back to unsigned
            OpType::JMP => self.pc = (self.pc as i32 + instruction.arg) as usize,
            OpType::NOP => self.pc += 1,
        }
    }

    // TODO add in an execute with a terminating condition?
}

fn part1(interpreter: &mut Interpreter) {
    // keep track of executed steps
    // any duplicate means we are in an infinite loop
    let mut executed = HashSet::new();

    while !executed.contains(&interpreter.pc) {
        executed.insert(interpreter.pc);
        interpreter.execute_step();
    }

    println!("{}", interpreter.accumulator)
}

// this is really icky...
fn part2(lines: &mut Vec<String>) {
    // program terminates if the pc reaches the end
    let terminate = lines.len();

    // check all possible instructions
    for (i, line) in lines.iter().enumerate() {
        // get the instruction
        let parsed_line: Vec<&str> = line.split(' ').collect();
        let op = OpType::from_str(parsed_line.get(0).unwrap()).unwrap();
        let arg = parsed_line.get(1).unwrap();
        if op == OpType::JMP || op == OpType::NOP {
            // create a local copy for the modification
            let mut lines = lines.clone();
            let new_instruction = match op {
                // switch opTypes
                OpType::JMP => "nop",
                OpType::NOP => "jmp",
                _ => "", // should never happen
            };

            let mut new_instruction = new_instruction.to_owned();
            new_instruction.push_str(" ");
            new_instruction.push_str(arg);

            std::mem::replace(&mut lines[i], new_instruction);
            // lines.replace(i, String::from(new_instruction));

            // create the updated interpreter
            let mut interpreter = Interpreter::new(&lines);

            let mut executed = HashSet::new();

            // perform the operation
            while !executed.contains(&interpreter.pc) && interpreter.pc != terminate {
                executed.insert(interpreter.pc);
                interpreter.execute_step();
            }

            // if we made it to the end, print the result
            if interpreter.pc == terminate {
                println!("{}", interpreter.accumulator);
                return;
            }
        }
    }
}

fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Error opening file!");
    BufReader::new(file)
        .lines()
        .map(|line| line.expect("Error reading line!"))
        .collect()
}

fn main() {
    let mut lines = read_lines("input.txt");
    let mut interpreter = Interpreter::new(&lines);
    part1(&mut interpreter);
    part2(&mut lines);
}
