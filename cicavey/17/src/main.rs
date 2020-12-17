use aoc;
use std::collections::HashMap;
use std::ops::RangeInclusive;

type Coord = (i64, i64, i64, i64);
type Space = HashMap<Coord, State>;

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Inactive,
    Active,
}

impl Default for State {
    fn default() -> Self { State::Inactive }
}

fn count_adjacent(c: Coord, s: &Space) -> usize {
    // Consider all adjacent spaces to c
    let mut count = 0;
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                for w in -1..=1 {
                    let n = (c.0 + x, c.1 + y, c.2 + z, c.3 + w);
                    // skip self
                    if n == c {
                        continue;
                    }
                    count += match s.get(&n).unwrap_or(&State::Inactive) {
                        State::Active => 1,
                        State::Inactive => 0,
                    }
                }
            }
        }
    }
    count
}

fn bounds(s: &Space, margin: Coord) -> (RangeInclusive<i64>, RangeInclusive<i64>, RangeInclusive<i64>, RangeInclusive<i64>) {
    let mut min = (0,0,0,0);
    let mut max = (0,0,0,0);
    for c in s.keys() {

        min.0 = i64::min(min.0, c.0);
        min.1 = i64::min(min.1, c.1);
        min.2 = i64::min(min.2, c.2);
        min.3 = i64::min(min.3, c.3);

        max.0 = i64::max(max.0, c.0);
        max.1 = i64::max(max.1, c.1);
        max.2 = i64::max(max.2, c.2);
        max.3 = i64::max(max.3, c.3);
    }

    min.0 -= margin.0;
    min.1 -= margin.1;
    min.2 -= margin.2;
    min.3 -= margin.3;

    max.0 += margin.0;
    max.1 += margin.1;
    max.2 += margin.2;
    max.3 += margin.3;

    (min.0..=max.0, min.1..=max.1, min.2..=max.2, min.3..=max.3)
}

fn step3(prev_space : &Space) -> Space {
    let mut next_space: Space = HashMap::new();
    let (xr, yr, zr, _) = bounds(prev_space, (1,1,1,1));
    for x in xr {
        for y in yr.clone() {
            for z in zr.clone() {
                let c = (x,y,z,0);
                let adj_count = count_adjacent(c, prev_space);
                let next_cell = match prev_space.get(&c).unwrap_or(&State::Inactive) {
                    State::Active => {
                        if adj_count == 2 || adj_count == 3 {
                            State::Active
                        } else {
                            State::Inactive
                        }
                        
                    },
                    State::Inactive => {
                        if adj_count == 3 {
                            State::Active
                        } else {
                            State::Inactive
                        }
                    },
                };
                next_space.insert(c, next_cell);
            }
        }
    }
    next_space
}

fn step4(prev_space : &Space) -> Space {
    let mut next_space: Space = HashMap::new();
    let (xr, yr, zr, wr) = bounds(prev_space, (1,1,1,1));
    for x in xr {
        for y in yr.clone() {
            for z in zr.clone() {
                for w in wr.clone() {
                    let c = (x,y,z,w);
                    let adj_count = count_adjacent(c, prev_space);
                    let next_cell = match prev_space.get(&c).unwrap_or(&State::Inactive) {
                        State::Active => {
                            if adj_count == 2 || adj_count == 3 {
                                State::Active
                            } else {
                                State::Inactive
                            }
                            
                        },
                        State::Inactive => {
                            if adj_count == 3 {
                                State::Active
                            } else {
                                State::Inactive
                            }
                        },
                    };
                    next_space.insert(c, next_cell);
                }
            }
        }
    }
    next_space
}

fn main() {
    let lines = aoc::lines_from_file("input.txt");

    let mut input_space: Space = HashMap::new();

    let mut y = 0;
    for line in lines {
        let mut x = 0;
        for c in line.chars() {
            if c == '#' {
                input_space.insert((x, y, 0, 0), State::Active);
            }
            x += 1;
        }
        y += 1;
    }

    let mut g = input_space.clone();
    for _ in 1..=6 {
        g = step3(&g);
    }
    let active_count = g.values().filter(|&v| *v == State::Active).count();
    dbg!(active_count);

    g = input_space.clone();
    for _ in 1..=6 {
        g = step4(&g);
    }
    let active_count = g.values().filter(|&v| *v == State::Active).count();
    dbg!(active_count);
}
