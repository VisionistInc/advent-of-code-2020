use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

struct Program {
    mask: String, // is making this a lifetime &str better?
    memory: HashMap<u64, u64>,
}

impl Program {
    fn new() -> Program {
        Program {
            mask: "".to_string(),
            memory: HashMap::new(),
        }
    }

    fn execute_instr(&mut self, instr: &String, volatile: bool) {
        let instr: Vec<&str> = instr.split(" = ").collect();
        let arg = instr[0];
        let val = instr[1];

        match &arg[0..3] {
            "mem" => {
                if !volatile {
                    self.mask_and_store(arg, val.parse().unwrap())
                } else {
                    self.mask_and_store_volatile(arg, val.parse().unwrap())
                }
            }
            "mas" => self.mask = val.to_owned(),
            _ => panic!("Invalid instruction {}", arg),
        }
    }

    fn mask_and_store(&mut self, arg: &str, val: u64) {
        let re = Regex::new(r"\[([0-9]+)]").unwrap();
        // get the number out of the address... eww
        let addr: u64 = re
            .captures(arg)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap();

        let ones_mask = self.mask.replace("X", "0");
        let ones_mask = u64::from_str_radix(&ones_mask, 2).unwrap();

        let zeros_mask = self.mask.replace("X", "1");
        let zeros_mask = u64::from_str_radix(&zeros_mask, 2).unwrap();

        let ones_res = ones_mask | val;
        // the result is the zeros mask of the ones mask result
        // doing it this chained way prevents the case where a don't care in the one's mask overrides a zero in the zeros' mask
        let result = zeros_mask & ones_res;

        self.memory.insert(addr, result);
    }

    fn mask_and_store_volatile(&mut self, arg: &str, val: u64) {
        let re = Regex::new(r"\[([0-9]+)]").unwrap();
        // get the number out of the address... eww
        let addr: u64 = re
            .captures(arg)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap();
        for addr in get_masked_addrs(addr, self.mask.clone()) {
            self.memory.insert(addr, val);
        }
    }
}

fn get_masked_addrs(addr: u64, mask: String) -> HashSet<u64> {
    let mask: Vec<char> = mask.chars().collect();
    let addr_string: Vec<char> = format!("{:0>36b}", addr).chars().collect();

    let masked_addr: Vec<char> = mask
        .iter()
        .enumerate()
        .map(|(i, char)| match char {
            '0' => addr_string[i],
            _ => *char,
        })
        .collect();

    permutations(masked_addr.iter().collect())
}

fn permutations(addr: String) -> HashSet<u64> {
    let mut perms = HashSet::new();
    if !addr.contains('X') {
        // let addr: String = addr.iter().collect();
        perms.insert(u64::from_str_radix(&addr, 2).unwrap());
        return perms;
    }

    perms.extend(permutations(addr.replacen('X', "1", 1)).iter());
    perms.extend(permutations(addr.replacen('X', "0", 1)).iter());

    perms
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

fn part1(lines: &Vec<String>) {
    let mut program = Program::new();
    for line in lines {
        program.execute_instr(line, false);
    }

    let result = program.memory.iter().fold(0, |acc, (_, val)| acc + val);
    println!("{}", result)
}

fn part2(lines: &Vec<String>) {
    let mut program = Program::new();
    for line in lines {
        program.execute_instr(line, true);
    }

    let result = program.memory.iter().fold(0, |acc, (_, val)| acc + val);
    println!("{}", result)
}

fn main() {
    let lines = read_lines("input.txt");
    part1(&lines);
    part2(&lines);
}
