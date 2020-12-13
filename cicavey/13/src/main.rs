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

    let ts = lines[0].parse::<u64>().unwrap();

    let mut buses: Vec<(u64, u64)> = lines[1]
        .split(",")
        .filter(|v| *v != "x")
        .map(|v| v.to_string().parse::<u64>().unwrap())
        .map(|v| (v, v - (ts % v)))
        .collect();

    buses.sort_by_key(|k| k.1);

    dbg!(buses[0].0 * buses[0].1);

    let mut bus_offsets = vec![];
    let mut offset = 0;
    for line in lines[1].split(",") {
        if line == "x" {
            offset += 1;
            continue;
        }
        let v = line.parse::<u64>().unwrap();

        // (v, offset)
        bus_offsets.push((v, offset));

        offset += 1;
    }

    let mut t: u64 = 0;
    let mut step = 1; //bus_offsets[0].0;

    let mut inc_step = 0;

    loop {
        let mut valid = true;
        let mut first = true;

        for bus in bus_offsets.iter().skip(inc_step) {
            // If this is the first thing AND it obeys the timestamp mod rule, increase the step and try for next
            // Each time a bus matches the step will multiple by the next bus id
            // There is most certainly a cleaner way to do this. Maybe checking the first bus outside of this loop
            if first && (t + bus.1) % bus.0 == 0 {
                inc_step += 1;
                step *= bus.0;
                valid = inc_step == bus_offsets.len();
                break;
            }

            first = false;
            valid = valid && ((t + bus.1) % bus.0 == 0)
        }
        if valid {
            break;
        }
        t += step;
    }
    dbg!(t);
}
