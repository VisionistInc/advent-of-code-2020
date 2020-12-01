use std::fs::File;
use std::io::{BufReader, BufRead};

fn part1(lines: &Vec<i32>) {
    for i in lines {
        for j in lines {
            if i + j == 2020 {
                println!("{}", i * j);
                return
            }
        }
    }
}

fn part2(lines: &Vec<i32>) {
    for i in lines {
        for j in lines {
            for k in lines {
                if i + j + k == 2020 {
                    println!("{}", i * j * k);
                    return
                }
            }
        }
    }
}

fn main() {
    let file = File::open("input.txt").expect("Can't open file!");
    let buf = BufReader::new(file);
    // TODO probably should improve error handling here
    let lines = buf.lines().map(|line| line.unwrap().parse::<i32>().unwrap()).collect();
    part1(&lines);
    part2(&lines);
}
