use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::collections::HashMap;
use euclid::*;

enum AoC {}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn count_trees(t: Vector2D<i32, AoC>, grid: &HashMap<Point2D<i32, AoC>, bool>, grid_dim: Size2D<i32, AoC>) -> u32 {
    let mut trees: u32 = 0;
    let mut p = point2(0, 0);
    while p.y < grid_dim.height {
        p += t;
        // Wrap around x coord
        p.x %= grid_dim.width;
        if grid.contains_key(&p) {
            trees += 1
        }
    }
    return trees;
}

fn main() {
    let lines = lines_from_file("input.txt");

    let grid_dim: Size2D<i32, AoC> = size2(lines.get(0).unwrap().len() as i32, lines.len() as i32);

    // for each line, convert trees into points
    let mut grid: HashMap<Point2D<i32, AoC>, bool>  = HashMap::new();
    let mut y = 0;
    for line in lines {
        let mut x = 0;
        for c in line.chars() {
            if c == '#' {
                grid.insert(point2(x, y), true);
            }
            x += 1;
        }
        y += 1;
    }

    let ts = vec!(
        vec2(1, 1),
        vec2(3, 1),
        vec2(5, 1),
        vec2(7, 1),
        vec2(1, 2)
    );

    let r : Vec<u32> = ts.into_iter().map(|t| count_trees(t, &grid, grid_dim)).collect();
    let part1 = r.get(1).unwrap();
    let acc : u32 = r.iter().product();

    dbg!(part1, acc);
}
