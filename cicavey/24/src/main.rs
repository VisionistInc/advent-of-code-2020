#[macro_use]
extern crate lazy_static;

use aoc;
use regex::Regex;
use std::cmp;
use std::collections::HashMap;

type HexCoord = (i32, i32, i32);
type HexVec = (i32, i32, i32);
type HexGrid = HashMap<HexCoord, i32>;

fn add(c: HexCoord, v: HexVec) -> HexCoord {
    (c.0 + v.0, c.1 + v.1, c.2 + v.2)
}

fn count_black_adj(input: &HexGrid, pos: HexCoord) -> u32 {
    lazy_static! {
        static ref dirs: Vec<HexVec> = vec![
            (1, 0, -1),
            (0, 1, -1),
            (1, -1, 0),
            (-1, 1, 0),
            (0, -1, 1),
            (-1, 0, 1)
        ];
    }

    let mut count: u32 = 0;
    for d in dirs.iter() {
        let n = add(pos, *d);
        let t = input.get(&n).unwrap_or(&0);
        count += *t as u32;
    }

    count
}

fn iterate(input: &HexGrid) -> HexGrid {
    let mut output = input.clone();
    let mut min = (0, 0, 0);
    let mut max = (0, 0, 0);
    for (p, t) in input {
        min.0 = cmp::min(min.0, p.0);
        min.1 = cmp::min(min.1, p.1);
        min.2 = cmp::min(min.2, p.2);

        max.0 = cmp::max(max.0, p.0);
        max.1 = cmp::max(max.1, p.1);
        max.2 = cmp::max(max.2, p.2);
    }

    min.0 -= 1;
    min.1 -= 1;
    min.2 -= 1;
    max.0 += 1;
    max.1 += 1;
    max.2 += 1;

    for x in min.0..=max.0 {
        for y in min.1..=max.1 {
            for z in min.2..=max.2 {
                let pos = (x, y, z);
                let t = *input.get(&pos).unwrap_or(&0);
                let c = count_black_adj(input, pos);

                let mut nv = t;

                // white tile, 2 black neighbors -> black
                if t == 0 && c == 2 {
                    nv = 1;
                    output.insert(pos, nv);
                    continue;
                }

                // black tile, 0 or more than 2 black nei -> white
                if t == 1 && (c == 0 || c > 2) {
                    nv = 0;
                    output.insert(pos, nv);
                    continue;
                }

                // output.insert(pos, nv);
            }
        }
    }

    output
}

fn main() {
    let mut grid: HexGrid = HashMap::new();

    let lines = aoc::lines_from_file("input.txt");
    let re: Regex = Regex::new(r"(ne|nw|se|sw|e|w)").unwrap();
    for line in lines {
        let mut pos = (0, 0, 0);

        for c in re.captures_iter(&line) {
            let dir = c.get(1).unwrap().as_str();

            let v: HexVec = match dir {
                "ne" => (1, 0, -1),
                "nw" => (0, 1, -1),
                "e" => (1, -1, 0),
                "w" => (-1, 1, 0),
                "se" => (0, -1, 1),
                "sw" => (-1, 0, 1),
                _ => (0, 0, 0),
            };

            pos = add(pos, v);
        }

        let t = grid.entry(pos).or_insert(0);
        // toggle between white (0) and black (1)
        *t = (*t + 1) % 2;
    }

    let sum: i32 = grid.values().sum();
    dbg!(sum);

    for day in 1..=100 {
        grid = iterate(&grid);
        let sum: i32 = grid.values().sum();
        println!("Day {}: {}", day, sum);
    }
}
