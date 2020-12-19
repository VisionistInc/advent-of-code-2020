use std::collections::HashSet;
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

fn part1(lines: &Vec<String>) {
    let earliest: isize = lines[0].parse().unwrap();
    let busses: Vec<isize> = lines[1]
        .split(',')
        .filter(|bus| *bus != "x")
        .map(|bus| bus.parse().unwrap())
        .collect();
    if let Some(best_bus) = busses.iter().fold(None, |best, bus| {
        if let Some(best_bus) = best {
            let best_wait: isize = (earliest % best_bus) - best_bus;
            let wait = (earliest % bus) - bus;

            if wait.abs() < best_wait.abs() {
                Some(bus)
            } else {
                Some(best_bus)
            }
        } else {
            Some(bus)
        }
    }) {
        let x = (earliest / best_bus) + 1; // integer division!
        println!("{}", ((best_bus * x) - earliest) * best_bus)
    }
}

fn part2(lines: &Vec<String>) {
    let busses: Vec<(usize, usize)> = lines[1]
        .split(',')
        .enumerate()
        .filter(|(_, bus)| *bus != "x")
        .map(|(i, bus)| (i, bus.parse::<usize>().unwrap()))
        .collect();

    let (_, first_arrival) = busses[0];

    let mut step_size = first_arrival;

    let mut already_arrived_busses: HashSet<usize> = HashSet::new();
    // start with the first one; we're already there
    already_arrived_busses.insert(first_arrival);

    let mut time: usize = first_arrival;
    loop {
        // get the busses that arrived here for the first time and multiply them to step size
        let arrived_busses: HashSet<usize> = busses
            .iter()
            .filter(|(offset, bus)| (time + offset) % bus == 0)
            .map(|(_, bus)| *bus)
            .collect();

        // if all busses arrived, we're done
        if arrived_busses.len() == busses.len() {
            println!("{}", time);
            return;
        }

        // get busses that arrived for the first time here and add them to the step size (thanks cavey)
        for bus in arrived_busses.difference(&already_arrived_busses) {
            step_size *= bus;
        }

        already_arrived_busses.extend(&arrived_busses);

        time += step_size;
    }
}

// fn part2_slow(lines: &Vec<String>) {
//     let busses: Vec<(usize, usize)> = lines[1]
//         .split(',')
//         .enumerate()
//         .filter(|(_, bus)| *bus != "x")
//         .map(|(i, bus)| (i, bus.parse::<usize>().unwrap()))
//         .collect();
//     let (_, first_arrival) = busses[0];
//     // zero would technically mean everyting matches, so start with the first arrival
//     let mut time: usize = first_arrival;
//     loop {
//         let mut done = true;
//         for (offset, bus) in &busses {
//             if (time + offset) % bus != 0 {
//                 done = false;
//                 break;
//             }
//             // let delta = (time % bus) - bus;
//             // dbg!(delta);
//         }
//         if done {
//             println!("{}", time);
//             return;
//         } else {
//             time += first_arrival;
//         }
//     }
// }

fn main() {
    let lines = read_lines("input.txt");
    part1(&lines);
    part2(&lines);
}
