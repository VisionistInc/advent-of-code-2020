use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Password {
    password: String,
    req_char: String, // could also be char, just don't want to bother with the conversion
    min: usize,       // usize helps with vector slicing
    max: usize,
}

fn part1(passwords: &Vec<Password>) {
    let num_working = passwords
        .iter()
        .filter(|password| {
            let num_req_chars = password.password.matches(&password.req_char).count();
            num_req_chars >= password.min && num_req_chars <= password.max
        })
        .count();
    println!("{}", num_working)
}

fn part2(passwords: &Vec<Password>) {
    let num_working = passwords
        .iter()
        .filter(|password| {
            let first = password.min - 1;
            let second = password.max - 1;

            // if either number is outside of the size of the word, it doesn't work!
            if first >= password.password.len() || second >= password.password.len() {
                return false;
            }

            let first_char = &password.password[first..first + 1];
            let second_char = &password.password[second..second + 1];
            (first_char == password.req_char || second_char == password.req_char)
                && first_char != second_char
        })
        .count();
    println!("{}", num_working)
}

fn main() {
    let file = File::open("input.txt").expect("Can't open file!");
    let buf = BufReader::new(file);
    let passwords: Vec<Password> = buf
        .lines()
        .map(|line| {
            // why can't i inline this guy in the pasred_line?
            let line = line.expect("Error reading line!");
            let parsed_line: Vec<&str> = line.split(' ').collect();

            // I wish vector destructuring was a thing...
            let numbers = parsed_line[0];
            let req_char = parsed_line[1];
            let password = parsed_line[2];

            // format is `1-2`, so drop the `-` and parse the numbers
            let numbers: Vec<usize> = numbers
                .split('-')
                .map(|num| num.parse::<usize>().expect("Error parsing number!"))
                .collect();
            let min = numbers[0];
            let max = numbers[1];

            // format is `c:` so drop the `:`
            let req_char = &req_char[0..1];

            Password {
                password: password.into(),
                req_char: req_char.into(),
                min,
                max,
            }
        })
        .collect();

    part1(&passwords);
    part2(&passwords);
}
