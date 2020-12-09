use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

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

fn check_match(buf: &[usize], target: usize) -> bool {
    for (i, val1) in buf.iter().enumerate() {
        for (j, val2) in buf.iter().enumerate() {
            // can't be the same number or the same value
            if i != j && val1 != val2 && val1 + val2 == target {
                return true;
            }
        }
    }
    false
}

fn part1(lines: &Vec<usize>) -> Option<usize> {
    for i in 0..lines.len() {
        let buf = &lines[i..25 + i];
        let target = *lines.get(25 + i).unwrap();
        if !check_match(buf, target) {
            println!("{}", target);
            return Some(target);
        }
    }
    None
}

fn part2(lines: &Vec<usize>, target: usize) {
    for i in 0..lines.len() {
        // get everything from this position to the end
        let buf = &lines[i..];
        let mut sequence: Vec<usize> = Vec::new();
        for num in buf {
            sequence.push(*num);
            let sum: usize = sequence.iter().sum();
            if sequence.len() >= 2 && sum == target {
                let max = sequence.iter().max().unwrap();
                let min = sequence.iter().min().unwrap();
                println!("{}", max + min);
                return;
            } else if sum > target {
                // this sequence didn't work... try the next one
                break;
            }
        }
    }
}

fn main() {
    let lines = read_lines("input.txt");
    let lines = lines.iter().map(|line| line.parse().unwrap()).collect();
    if let Some(target) = part1(&lines) {
        part2(&lines, target);
    }
}
