#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

struct Entry {
    pw: String,
    c: char,
    min: usize,
    max: usize,
}

fn check1(e: &Entry) -> bool {
    let mut count: usize = 0;
    for pc in e.pw.chars() {
        if pc == e.c {
            count += 1;
        }
    }
    count >= e.min && count <= e.max
}

fn check2(e: &Entry) -> bool {
    (e.pw.chars().nth(e.min-1).unwrap() == e.c) ^ (e.pw.chars().nth(e.max-1).unwrap() == e.c)
}

fn parse(s: String) -> Entry {

    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    }

    let caps = RE.captures(&s).unwrap();

    let min = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let max = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
    let c = caps.get(3).unwrap().as_str().chars().next().unwrap();
    let pw = caps.get(4).unwrap().as_str();

    Entry { pw: pw.to_string(), c , min, max}
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {

    let lines = lines_from_file("input.txt");

    let entries: Vec<Entry> = lines.into_iter().map(parse).collect();

    let part1 = entries.iter().map(check1).filter(|b| *b).count();
    let part2 = entries.iter().map(check2).filter(|b| *b).count();

    println!("{} {}", part1, part2);
}
