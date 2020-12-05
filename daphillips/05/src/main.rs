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

// the "upper" and "lower" indication is just a standard binary encoding:
// the "upper" represents 1 and "lower" 0
fn dir_to_binary(directions: &str, upper: char, lower: char) -> u32 {
    let size = directions.len() - 1;
    directions
        .chars()
        .map(|dir| {
            if dir == upper {
                0b1
            } else if dir == lower {
                0b0
            } else {
                panic!(
                    "Invalid direction {}! Valid options include {} and {}",
                    dir, upper, lower
                )
            }
        })
        .enumerate()
        .map(|(pos, bit)| bit << (size - pos))
        .fold(0, |acc, bit| bit + acc)
}

fn get_seat_id(pass: &String) -> u32 {
    let row = dir_to_binary(&pass[..7], 'B', 'F');
    let col = dir_to_binary(&pass[7..], 'R', 'L');
    return row * 8 + col;
}

fn part1(passes: &Vec<String>) {
    if let Some(max) = passes.iter().map(get_seat_id).max() {
        println!("{}", max)
    } else {
        panic!("No max found!")
    }
}

fn part2(passes: &Vec<String>) {
    let mut passes: Vec<u32> = passes.iter().map(get_seat_id).collect();
    // we don't need a stable sort, so we can possibly be a little faster with an unstable one
    passes.sort_unstable();

    // pop the first element from the iterator so we can safely compare with the previous element
    for (i, pass) in passes.iter().enumerate() {
        // skip the first iteration so we don't panic
        if i != 0 {
            if pass - passes[i - 1] == 2 {
                // found our seat!
                println!("{}", pass - 1);
                return;
            }
        }
    }
}

fn main() {
    let passes = read_lines("input.txt");
    part1(&passes);
    part2(&passes);
}
