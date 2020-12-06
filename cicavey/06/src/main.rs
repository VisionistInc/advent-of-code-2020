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

fn part1() {
    let lines = lines_from_file("input.txt");

    let mut group: u32 = 0;
    let mut qsum: u32 = 0;
    for line in lines {
        if line.is_empty() {
            qsum += group.count_ones();
            // group done
            group = 0;
        }

        for c in line.chars() {
            let q = (c as u8) - 97;
            group = group | (1 << q);
            // println!("{}", q);
        }
    }
    qsum += group.count_ones();

    dbg!(qsum);
}

fn part2() {
    let lines = lines_from_file("input.txt");

    let mut group: u32 = 0xFFFFFFFF;
    let mut qsum: u32 = 0;

    for line in lines {
        if line.is_empty() {
            qsum += group.count_ones();
            // group done
            group = 0xFFFFFFFF;

            continue;
        }

        let mut p: u32 = 0;

        for c in line.chars() {
            let q = (c as u8) - 97;
            p = p | (1 << q);
        }

        // bitmask and all the peoeple into group accumulator
        group = group & p;
    }
    qsum += group.count_ones();

    dbg!(qsum);
}

fn main() {
    part1();
    part2();
}
