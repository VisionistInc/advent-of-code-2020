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

const PRE: usize = 25;

fn find_pair(s: [u64; PRE], sum: u64) -> bool {
    for x in 0..PRE {
        for y in x..PRE {
            let xv = s[x];
            let yv = s[y];
            if xv == yv {
                continue;
            }
            if xv + yv == sum {
                return true;
            }
        }
    }

    false
}

fn main() {
    let mut prev: [u64; PRE] = [0; PRE];
    let mut first = 0;
    let mut count = 0;

    let lines = lines_from_file("input.txt");

    let values: Vec<u64> = lines
        .into_iter()
        .map(|l| l.parse::<u64>().unwrap())
        .collect();

    // Read preable
    for v in (&values).into_iter().take(PRE) {
        prev[count] = *v;
        count += 1;
    }

    let mut target = 0;

    for v in (&values).into_iter().skip(PRE) {
        let bv = *v;
        let pair = find_pair(prev, bv);

        if !pair {
            target = bv;
            dbg!(v);
            break;
        }

        prev[first] = bv;
        first = (first + 1) % PRE;
    }

    let vi = values.iter().position(|&v| v == target).unwrap();

    dbg!(vi);

    // find a contiguous set in the first 618 lines that sum to our target
    'done: for start in 0..=vi {
        for end in start + 1..=vi {
            let mut sum: u64 = 0;

            for i in start..end {
                sum += values[i];

                if sum == target {
                    let max = (&values)
                        .into_iter()
                        .skip(start)
                        .take(end - start)
                        .max()
                        .unwrap();
                    let min = (&values)
                        .into_iter()
                        .skip(start)
                        .take(end - start)
                        .min()
                        .unwrap();

                    dbg!(start, end, sum, target, min + max);

                    break 'done;
                }

                if sum > target {
                    break;
                }
            }
        }
    }
}
