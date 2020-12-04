use reduce::Reduce;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

// adapted from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
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

fn part1(lines: &Vec<Vec<char>>) {
    let mut pos = 0;
    let mut num_trees_hit = 0;
    for line in lines {
        if line[pos] == '#' {
            num_trees_hit += 1
        }

        pos = (pos + 3) % line.len()
    }

    println!("{}", num_trees_hit)
}

fn part2(lines: &Vec<Vec<char>>) {
    let shifts = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut all_trees = Vec::new();

    for (x_shift, y_shift) in shifts {
        let mut x_pos = 0;
        let mut y_pos = 0;
        let mut num_trees_hit = 0;
        for line in lines {
            // first check if this is a row we should evaluate
            if y_pos % y_shift == 0 {
                // check for collision
                if line[x_pos] == '#' {
                    num_trees_hit += 1
                }

                // update x position and wrap
                x_pos = (x_pos + x_shift) % line.len()
            }
            y_pos += 1
        }
        all_trees.push(num_trees_hit)
    }

    // into_iter moves the values so that we can use reduce
    // unsigned to avoid overflow (could also use 64 or 128 here)
    // should never be empty so we can safely unwrap
    let trees_product: u32 = all_trees.into_iter().reduce(|a, b| a * b).unwrap();

    println!("{}", trees_product)
}

fn main() {
    let lines = read_lines("input.txt");

    // map the lines from `String`s to `Vec<char>`s for easier indexing
    let lines: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    part1(&lines);
    part2(&lines);
}
