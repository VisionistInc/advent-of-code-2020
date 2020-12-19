use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate(i64, i64, i64);

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate4D(i64, i64, i64, i64);

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

fn get_neighbors(pos: &Coordinate) -> Vec<Coordinate> {
    let Coordinate(x, y, z) = *pos;

    let mut neighbors = Vec::new();
    for x in vec![x - 1, x, x + 1] {
        for y in vec![y - 1, y, y + 1] {
            for z in vec![z - 1, z, z + 1] {
                if Coordinate(x, y, z) != *pos {
                    neighbors.push(Coordinate(x, y, z))
                }
            }
        }
    }

    neighbors
}

fn active_neighbors_count(pos: &Coordinate, active_cubes: &Vec<Coordinate>) -> usize {
    // every neighbor possibility
    // probably should have generated these permutations, but this was simpler (even if it was longer)
    let neighbors = get_neighbors(pos);

    return neighbors
        .iter()
        .filter(|neighbor| active_cubes.contains(neighbor))
        .count();
}

fn iterate(active_cubes: &Vec<Coordinate>) -> Vec<Coordinate> {
    let mut next_step: Vec<Coordinate> = Vec::new();

    // use a set for potential neighbors so that we don't have to worry about duplicates
    let mut potential_active_neighbors: HashSet<Coordinate> = HashSet::new();

    for active_cube in active_cubes {
        // todo check if this should stay alive
        let neighbors_count = active_neighbors_count(active_cube, active_cubes);
        if neighbors_count == 2 || neighbors_count == 3 {
            next_step.push(*active_cube);
        }

        // get all inactive neighbors and put them in the active neighbors set
        potential_active_neighbors.extend(
            get_neighbors(active_cube)
                .iter()
                .filter(|neighbor| !active_cubes.contains(neighbor)),
        );
    }

    // check the potential neighbors to see if any should become active
    for neighbor in potential_active_neighbors {
        if active_neighbors_count(&neighbor, active_cubes) == 3 {
            next_step.push(neighbor);
        }
    }

    next_step
}

fn part1(active_cubes: &Vec<Coordinate>, num_iterations: usize) {
    let mut active_cubes = active_cubes.clone();
    for i in 1..=num_iterations {
        println!("Iteration {} of {}", i, num_iterations);
        active_cubes = iterate(&active_cubes);
    }

    println!("{}", active_cubes.len())
}

fn get_neighbors_4d(pos: &Coordinate4D) -> Vec<Coordinate4D> {
    let Coordinate4D(x, y, z, w) = *pos;

    let mut neighbors = Vec::new();
    for x in vec![x - 1, x, x + 1] {
        for y in vec![y - 1, y, y + 1] {
            for z in vec![z - 1, z, z + 1] {
                for w in vec![w - 1, w, w + 1] {
                    if Coordinate4D(x, y, z, w) != *pos {
                        neighbors.push(Coordinate4D(x, y, z, w))
                    }
                }
            }
        }
    }

    neighbors
}

// I really should just try to make a union or make a more generic way of handling this near-duplicate code, but i'm lazy
fn active_neighbors_count_4d(pos: &Coordinate4D, active_cubes: &Vec<Coordinate4D>) -> usize {
    // every neighbor possibility
    // probably should have generated these permutations, but this was simpler (even if it was longer)
    let neighbors = get_neighbors_4d(pos);

    return neighbors
        .iter()
        .filter(|neighbor| active_cubes.contains(neighbor))
        .count();
}

fn iterate_4d(active_cubes: &Vec<Coordinate4D>) -> Vec<Coordinate4D> {
    let mut next_step: Vec<Coordinate4D> = Vec::new();

    // use a set for potential neighbors so that we don't have to worry about duplicates
    let mut potential_active_neighbors: HashSet<Coordinate4D> = HashSet::new();

    for active_cube in active_cubes {
        // todo check if this should stay alive
        let neighbors_count = active_neighbors_count_4d(active_cube, active_cubes);
        if neighbors_count == 2 || neighbors_count == 3 {
            next_step.push(*active_cube);
        }

        // get all inactive neighbors and put them in the active neighbors set
        potential_active_neighbors.extend(
            get_neighbors_4d(active_cube)
                .iter()
                .filter(|neighbor| !active_cubes.contains(neighbor)),
        );
    }

    // check the potential neighbors to see if any should become active
    for neighbor in potential_active_neighbors {
        if active_neighbors_count_4d(&neighbor, active_cubes) == 3 {
            next_step.push(neighbor);
        }
    }

    next_step
}

fn part2(active_cubes: &Vec<Coordinate4D>, num_iterations: usize) {
    let mut active_cubes = active_cubes.clone();
    for i in 1..=num_iterations {
        println!("Iteration {} of {}", i, num_iterations);
        active_cubes = iterate_4d(&active_cubes);
    }

    println!("{}", active_cubes.len())
}

fn main() {
    let lines = read_lines("input.txt");
    let mut active_cubes: Vec<Coordinate> = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        line.match_indices('#')
            .for_each(|(x, _)| active_cubes.push(Coordinate(x as i64, y as i64, 0)))
    }

    let mut active_cubes_4d: Vec<Coordinate4D> = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        line.match_indices('#')
            .for_each(|(x, _)| active_cubes_4d.push(Coordinate4D(x as i64, y as i64, 0, 0)))
    }

    part1(&active_cubes, 6);
    part2(&active_cubes_4d, 6);
}
