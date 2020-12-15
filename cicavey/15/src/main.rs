// use std::fs::File;
// use std::io::{BufRead, BufReader};
// use std::path::Path;
// use std::collections::HashMap;

// fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
//     let file = File::open(filename).expect("no such file");
//     let buf = BufReader::new(file);
//     buf.lines()
//         .map(|l| l.expect("Could not parse line"))
//         .collect()
// }

use std::collections::HashMap;

use aoc;

type Entry = (u64, u64);

fn main() {
    let input = "14,8,16,0,1,17";
    // let input = "3,1,2";

    let values: Vec<u64> = input.split(",").map(|v| v.parse().unwrap()).collect();

    let mut t = 1;

    let mut mem : HashMap<u64, Entry> = HashMap::new();

    let mut prev = 0;

    for v in values {
        prev = v;
        mem.insert(v, (t, t));
        t += 1;
    }

    loop {
        match mem.get(&prev) {
            Some(e) => {
                // first time, say 0
                let apart = e.0 - e.1;
                if apart == 0 {
                    prev = 0;
                } else {
                    prev = apart;
                }
                mem.entry(prev)
                    .and_modify(|e| { e.1 = e.0; e.0 = t; })
                    .or_insert((t, t));
            }
            None => {
                mem.insert(0, (t, t));
            }
        }

        t += 1;

        if t == 2021 {
            println!("last spoken = {}", prev);
        }

        if t > 30_000_000 {
            break;
        }
    }

    println!("last spoken = {}", prev);
}
