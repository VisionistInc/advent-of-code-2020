use bit_vec::BitVec;
use std::cmp;
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

fn main() {
    let lines = lines_from_file("input.txt");

    let mut max_id = 0;

    let mut seats = BitVec::from_elem(128 * 8, false);

    for line in lines {
        let mut min = 0;
        let mut max = 127;

        for c in line.chars().take(7) {
            let w = (max - min) / 2 + 1;
            if c == 'F' {
                max -= w
            } else {
                min += w
            }
        }

        let row = min;

        min = 0;
        max = 7;

        for c in line.chars().skip(7) {
            let w = (max - min) / 2 + 1;
            if c == 'L' {
                max -= w
            } else {
                min += w
            }
        }

        let col = min;

        seats.set(row * 8 + col, true);

        let id = row * 8 + col;

        max_id = cmp::max(max_id, id);
    }

    dbg!(max_id);

    'done: for row in 0..128 {
        for (i, c) in seats.iter().skip(row * 8).take(8).enumerate() {
            if row > 4 && !c {
                dbg!(row * 8 + i);
                break 'done;
            }
        }
    }
}
