use euclid::*;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

enum AoC {}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum GridState {
    Occupied,
    Unoccupied,
    Floor,
}

impl Default for GridState {
    fn default() -> Self {
        GridState::Floor
    }
}

impl fmt::Display for GridState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GridState::Occupied => write!(f, "#"),
            GridState::Unoccupied => write!(f, "L"),
            GridState::Floor => write!(f, "."),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Grid {
    w: i32,
    h: i32,
    data: HashMap<Point2D<i32, AoC>, GridState>,
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            w: 0,
            h: 0,
            data: HashMap::new(),
        }
    }

    pub fn display(&self) {
        for y in 0..self.h {
            for x in 0..self.w {
                match self.data.get(&point2(x, y)) {
                    Some(s) => print!("{}", s),
                    _ => {}
                }
            }
            println!();
        }
    }

    pub fn count_adjacent(&self, p: Point2D<i32, AoC>, kind: GridState) -> u8 {
        let adj: Vec<Vector2D<i32, AoC>> = vec![
            vec2(1, 0),
            vec2(1, 1),
            vec2(0, 1),
            vec2(-1, 1),
            vec2(-1, 0),
            vec2(-1, -1),
            vec2(0, -1),
            vec2(1, -1),
        ];

        let mut sum = 0;
        for a in adj {
            let t = p + a;
            let v = self.data.get(&t);
            if *v.unwrap_or(&GridState::Floor) == kind {
                sum += 1
            }
        }

        sum
    }

    pub fn count_adjacent_visible(&self, p: Point2D<i32, AoC>, kind: GridState) -> u8 {
        let adj: Vec<Vector2D<i32, AoC>> = vec![
            vec2(1, 0),
            vec2(1, 1),
            vec2(0, 1),
            vec2(-1, 1),
            vec2(-1, 0),
            vec2(-1, -1),
            vec2(0, -1),
            vec2(1, -1),
        ];

        let mut sum = 0;
        'outer: for a in adj {
            let mut t = p;
            loop {
                t = t + a;
                let v = self.data.get(&t);
                match v {
                    Some(state) => match state {
                        GridState::Floor => continue,
                        other => {
                            if kind == *other {
                                sum += 1;
                            }
                            continue 'outer;
                        }
                    },
                    _ => continue 'outer,
                }
            }
        }

        sum
    }

    pub fn iterate(&self) -> Grid {
        let mut target = self.clone();

        for (&p, s) in self.data.iter() {
            match s {
                GridState::Unoccupied => {
                    if self.count_adjacent(p, GridState::Occupied) == 0 {
                        target.data.insert(p, GridState::Occupied);
                    }
                }
                GridState::Occupied => {
                    if self.count_adjacent(p, GridState::Occupied) >= 4 {
                        target.data.insert(p, GridState::Unoccupied);
                    }
                }
                _ => {}
            }
        }

        target
    }

    pub fn iterate2(&self) -> Grid {
        let mut target = self.clone();

        for (&p, s) in self.data.iter() {
            match s {
                GridState::Unoccupied => {
                    if self.count_adjacent_visible(p, GridState::Occupied) == 0 {
                        target.data.insert(p, GridState::Occupied);
                    }
                }
                GridState::Occupied => {
                    if self.count_adjacent_visible(p, GridState::Occupied) >= 5 {
                        target.data.insert(p, GridState::Unoccupied);
                    }
                }
                _ => {}
            }
        }

        target
    }

    pub fn occupied(&self) -> usize {
        self.data
            .values()
            .filter(|&v| v == &GridState::Occupied)
            .count()
    }
}

fn main() {
    let mut grid: Grid = Grid::new();

    let lines = lines_from_file("input.txt");

    let mut y = 0;
    for line in lines {
        let mut x = 0;
        for c in line.chars() {
            match c {
                'L' => grid.data.insert(point2(x, y), GridState::Unoccupied),
                '#' => grid.data.insert(point2(x, y), GridState::Occupied),
                _ => grid.data.insert(point2(x, y), GridState::Floor),
            };
            x += 1;
        }
        grid.w = x;
        y += 1;
    }
    grid.h = y;

    let og = grid.clone();

    loop {
        let new_grid = grid.iterate();

        if new_grid == grid {
            break;
        }
        grid = new_grid;
    }

    dbg!(grid.occupied());

    grid = og;

    loop {
        let new_grid = grid.iterate2();

        if new_grid == grid {
            break;
        }
        grid = new_grid;
    }

    dbg!(grid.occupied());
}
