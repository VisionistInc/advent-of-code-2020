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

fn rotate_left(v: Vector2D<i64, AoC>, deg: i64) -> Vector2D<i64, AoC> {
    match deg % 360 {
        90 => vec2(-v.y, v.x),
        180 => vec2(-v.x, -v.y),
        270 => vec2(v.y, -v.x),
        _ => v.clone(),
    }
}

fn rotate_right(v: Vector2D<i64, AoC>, deg: i64) -> Vector2D<i64, AoC> {
    match deg % 360 {
        90 => vec2(v.y, -v.x),
        180 => vec2(-v.x, -v.y),
        270 => vec2(-v.y, v.x),
        _ => v.clone(),
    }
}

fn main() {
    let lines = lines_from_file("input.txt");

    //     N
    //   W   e
    //     S
    let N: Vector2D<i64, AoC> = vec2(0, 1);
    let S: Vector2D<i64, AoC> = vec2(0, -1);
    let E: Vector2D<i64, AoC> = vec2(1, 0);
    let W: Vector2D<i64, AoC> = vec2(-1, 0);

    let mut pos: Point2D<i64, AoC> = point2(0, 0);
    let mut dir: Vector2D<i64, AoC> = E.clone();
    for line in &lines {
        let v = line[1..].parse::<i64>().unwrap();
        match &line[0..1] {
            "N" => {
                pos += N * v;
            }
            "S" => {
                pos += S * v;
            }
            "E" => {
                pos += E * v;
            }
            "W" => {
                pos += W * v;
            }
            "L" => {
                dir = rotate_left(dir, v);
            }
            "R" => {
                dir = rotate_right(dir, v);
            }
            "F" => {
                pos += dir * v;
            }
            _ => {}
        }
    }

    dbg!(pos.x.abs() + pos.y.abs());
    // part 2
    let mut pos: Point2D<i64, AoC> = point2(0, 0);
    let mut way: Vector2D<i64, AoC> = vec2(10, 1);
    for line in &lines {
        let v = line[1..].parse::<i64>().unwrap();
        match &line[0..1] {
            "N" => {
                way += vec2(0, v);
            }
            "S" => {
                way += vec2(0, -v);
            }
            "E" => {
                way += vec2(v, 0);
            }
            "W" => {
                way += vec2(-v, 0);
            }
            "L" => {
                way = rotate_left(way, v);
            }
            "R" => {
                way = rotate_right(way, v);
            }
            "F" => {
                pos += way * v;
            }
            _ => {}
        }
    }

    dbg!(pos.x.abs() + pos.y.abs());
}
