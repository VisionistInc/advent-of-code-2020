use std::collections::HashMap;
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
    let mut values: Vec<u64> = lines
        .into_iter()
        .map(|l| l.parse::<u64>().unwrap())
        .collect();
    values.sort();

    // Add entry for your device
    values.insert(0, 0);
    let max = values.last().unwrap() + 3;
    values.push(max);

    // Calculate all differences
    let diff: Vec<u64> = values.windows(2).map(|w| w[1] - w[0]).collect();
    // Histogram differences
    let mut hist = HashMap::new();
    for v in diff {
        *hist.entry(v).or_insert(0) += 1;
    }
    let magic = hist.get(&1).unwrap() * hist.get(&3).unwrap();
    dbg!(magic);

    let mut lookup = vec![0; (max + 1) as usize];
    lookup[0] = 1;

    // For every value calculate the total number of possible routes from the previous three values
    // Use lookup table
    // Full disclosure - I tried the brute force / recursive version and it obviously didn't work for larger case
    // Attempted to build a graph, too much work. Considered the route summing after skimming other solutions
    for v in values.iter_mut().skip(1) {
        let i = *v as usize;
        // saturating_sub is the bounded sub

        let mut sum: u64 = 0;
        for k in i.saturating_sub(3)..i {
            sum += *lookup.get(k).unwrap_or(&0);
        }
        lookup[i] = sum;
    }

    dbg!(lookup.last().unwrap());
}
