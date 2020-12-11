use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

struct Coordinate(usize, usize);

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

fn add_possible_neighbor(
    neighbors: &mut Vec<Coordinate>,
    coord: Coordinate,
    seats: &Vec<Vec<char>>,
    x_delta: isize,
    y_delta: isize,
    part2: bool,
) {
    let Coordinate(i, j) = coord;
    if let Some(line) = seats.get(i) {
        if let Some(neighbor) = line.get(j) {
            if *neighbor == '#' {
                neighbors.push(coord);
            } else if part2 && *neighbor == '.' {
                // for part 2, we check the nearest neighbor; ignoring the floor
                // so we need to recursively check in the continuing direction
                // this feels really yucky... but i was just writing to get it done
                let new_x = (i as isize) + x_delta;
                let new_y = (j as isize) + y_delta;
                if new_x >= 0 && new_y >= 0 {
                    add_possible_neighbor(
                        neighbors,
                        Coordinate(new_x as usize, new_y as usize),
                        seats,
                        x_delta,
                        y_delta,
                        part2,
                    )
                }
            }
        }
    }
}

fn count_occupied_neighbors(pos: &Coordinate, seats: &Vec<Vec<char>>, part2: bool) -> usize {
    let Coordinate(x, y) = pos;
    let x = *x;
    let y = *y;

    let mut open_neighbors = Vec::new();

    if x != 0 {
        add_possible_neighbor(
            &mut open_neighbors,
            Coordinate(x - 1, y),
            seats,
            -1,
            0,
            part2,
        );
        add_possible_neighbor(
            &mut open_neighbors,
            Coordinate(x - 1, y + 1),
            seats,
            -1,
            1,
            part2,
        );
    }

    if y != 0 {
        add_possible_neighbor(
            &mut open_neighbors,
            Coordinate(x, y - 1),
            seats,
            0,
            -1,
            part2,
        );
        add_possible_neighbor(
            &mut open_neighbors,
            Coordinate(x + 1, y - 1),
            seats,
            1,
            -1,
            part2,
        );
    }

    if x != 0 && y != 0 {
        add_possible_neighbor(
            &mut open_neighbors,
            Coordinate(x - 1, y - 1),
            seats,
            -1,
            -1,
            part2,
        );
    }

    add_possible_neighbor(
        &mut open_neighbors,
        Coordinate(x + 1, y),
        seats,
        1,
        0,
        part2,
    );
    add_possible_neighbor(
        &mut open_neighbors,
        Coordinate(x, y + 1),
        seats,
        0,
        1,
        part2,
    );
    add_possible_neighbor(
        &mut open_neighbors,
        Coordinate(x + 1, y + 1),
        seats,
        1,
        1,
        part2,
    );

    open_neighbors.len()
}

fn iterate(seats: &mut Vec<Vec<char>>, tolerance: usize, part2: bool) -> Vec<Vec<char>> {
    let mut new_seats = seats.clone();
    for (i, row) in seats.iter().enumerate() {
        for (j, seat) in row.iter().enumerate() {
            let occupied_neighbors = count_occupied_neighbors(&Coordinate(i, j), seats, part2);
            // could in theory make these checks more generic, but we don't need to do that now
            match seat {
                'L' => {
                    if occupied_neighbors == 0 {
                        new_seats[i][j] = '#';
                    }
                }
                '#' => {
                    if occupied_neighbors >= tolerance {
                        new_seats[i][j] = 'L';
                    }
                }
                _ => (),
            }
        }
    }

    new_seats
}

fn part1(seats: &Vec<Vec<char>>) {
    let mut seats = seats.clone();
    let mut old_seats = Vec::new();

    while seats != old_seats {
        old_seats = seats.clone();
        seats = iterate(&mut seats, 4, false);
    }

    let occupied_seats: usize = seats
        .iter()
        .map(|row| row.iter().filter(|seat| **seat == '#').count())
        .sum();
    println!("{}", occupied_seats)
}

fn part2(seats: &Vec<Vec<char>>) {
    let mut seats = seats.clone();
    let mut old_seats = Vec::new();

    while seats != old_seats {
        old_seats = seats.clone();
        seats = iterate(&mut seats, 5, true);
    }

    let occupied_seats: usize = seats
        .iter()
        .map(|row| row.iter().filter(|seat| **seat == '#').count())
        .sum();
    println!("{}", occupied_seats)
}

fn main() {
    let lines: Vec<Vec<char>> = read_lines("input.txt")
        .iter()
        .map(|line| line.chars().collect())
        .collect();
    part1(&lines);
    part2(&lines);
}
