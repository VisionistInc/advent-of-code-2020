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

fn part1(lines: &Vec<usize>) {
    let mut one_deltas = 0;
    // there's always the three delta of the charger at the beginnig
    let mut three_deltas = 1;

    let mut previous: usize = 0;

    // pop the first element from the iterator so we can safely compare with the previous element
    for num in lines {
        // skip the first iteration so we don't panic
        let delta = num - previous;
        match delta {
            1 => one_deltas += 1,
            3 => three_deltas += 1,
            2 => (), // don't care about 2
            _ => panic!("Invalid delta {}!", delta),
        }
        previous = *num;
    }

    println!("{}", one_deltas * three_deltas)
}

fn part2(lines: &Vec<usize>) {
    let mut lines = lines.clone();
    // add dummy 0 so that we can calculate the paths for the first 3 (might have a 1, 2, or 3)
    lines.insert(0, 0);

    let mut paths: Vec<usize> = vec![0; lines.len()];
    paths[0] = 1;
    for (i, num) in lines.iter().enumerate() {
        let paths_here = paths[i];

        // had trouble making an iter() multi-use, so i'm just creating them here
        if let Some(idx) = lines.iter().position(|&n| n == num + 1) {
            paths[idx] += paths_here
        }

        if let Some(idx) = lines.iter().position(|&n| n == num + 2) {
            paths[idx] += paths_here
        }

        if let Some(idx) = lines.iter().position(|&n| n == num + 3) {
            paths[idx] += paths_here
        }
    }
    println!("{}", paths.last().unwrap())
}

fn main() {
    let lines = read_lines("input.txt");
    let mut lines: Vec<usize> = lines.iter().map(|line| line.parse().unwrap()).collect();
    // just need a linear order; "stable" preserves order of duplicates (we don't care about that here)
    lines.sort_unstable();
    part1(&lines);
    part2(&lines);
}
