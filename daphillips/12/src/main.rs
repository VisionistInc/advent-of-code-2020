use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

struct Coordinate(isize, isize);


#[derive(Copy, Clone, PartialEq)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
    LEFT,
    RIGHT,
    FORWARD,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "N" => Ok(Direction::NORTH),
            "S" => Ok(Direction::SOUTH),
            "E" => Ok(Direction::EAST),
            "W" => Ok(Direction::WEST),
            "L" => Ok(Direction::LEFT),
            "R" => Ok(Direction::RIGHT),
            "F" => Ok(Direction::FORWARD),
            _ => Err(()),
        }
    }
}

struct Ship {
    cur_bearing: isize, // could also be direction, but just using a number limits conversion needs
    position: Coordinate,
    // the waypoint's coordinates are just a delta to the ship!
    waypoint: Coordinate,
}

impl Ship {
    // part1's update
    fn update_pos(&mut self, direction: Direction, distance: isize) {
        let Coordinate(x, y) = self.position;
        match direction {
            Direction::NORTH => self.position = Coordinate(x, y + distance),
            Direction::SOUTH => self.position = Coordinate(x, y - distance),
            Direction::EAST => self.position = Coordinate(x + distance, y),
            Direction::WEST => self.position = Coordinate(x - distance, y),
            // use 360 - distance so that we don't have to worry about negative directions
            Direction::LEFT => self.cur_bearing = (self.cur_bearing + (360 - distance)) % 360,
            Direction::RIGHT => self.cur_bearing = (self.cur_bearing + distance) % 360,
            Direction::FORWARD => self.move_forward(distance),
        }
    }

    fn move_forward(&mut self, distance: isize) {
        match self.cur_bearing {
            0 => self.update_pos(Direction::NORTH, distance),
            90 => self.update_pos(Direction::EAST, distance),
            180 => self.update_pos(Direction::SOUTH, distance),
            270 => self.update_pos(Direction::WEST, distance),
            _ => panic!("Invalid cur_bearing {}", self.cur_bearing)
        }
    }

    fn update_via_waypoint(&mut self, direction: Direction, distance: isize) {
        let Coordinate(x, y) = self.waypoint;
        match direction {
            Direction::NORTH => self.waypoint = Coordinate(x, y + distance),
            Direction::SOUTH => self.waypoint = Coordinate(x, y - distance),
            Direction::EAST => self.waypoint = Coordinate(x + distance, y),
            Direction::WEST => self.waypoint = Coordinate(x - distance, y),
            // use 360 - distance so that we don't have to worry about negative directions
            Direction::LEFT => self.rotate_waypoint((360 - distance) % 360),
            Direction::RIGHT => self.rotate_waypoint(distance % 360),
            Direction::FORWARD => {
                let Coordinate(x_ship, y_ship) = self.position;
                self.position = Coordinate(x_ship + (distance * x), y_ship + (distance * y));
            },
        }
    }

    fn rotate_waypoint(&mut self, new_bearing: isize) {
        let Coordinate(x, y) = self.waypoint;
        match new_bearing {
            // nothing
            0 => (),
            // swap and negate new y
            90 => self.waypoint = Coordinate(y, -x),
            // just negate
            180 => self.waypoint = Coordinate(-x, -y),
            // swap values and negate new x
            270 => self.waypoint = Coordinate(-y, x),
            _ => panic!("Invalid new_bearing {}", new_bearing),
        }
    }
}

fn read_lines<P>(filename: P) -> Vec<String>
    where P: AsRef<Path>, {
    let file = File::open(filename).expect("Error opening file!");
    BufReader::new(file).lines().map(|line| line.expect("Error reading line!")).collect()
}

fn part1(lines: &Vec<String>) {
    let mut ship = Ship {
        position: Coordinate(0,0),
        cur_bearing: 90,
        waypoint: Coordinate(0,0) // not used for part 1
    };
    for line in lines {
        let distance: isize = (&line[1..]).parse().unwrap();
        let direction = Direction::from_str(&line[0..1]).unwrap();
        ship.update_pos(direction, distance);
    }

    let Coordinate(x, y) = ship.position;

    let manhattan_dist = x.abs() + y.abs();
    println!("{}", manhattan_dist);
}

fn part2(lines: &Vec<String>) {
    let mut ship = Ship {
        position: Coordinate(0,0),
        cur_bearing: 90,
        waypoint: Coordinate(10,1)
    };

    for line in lines {
        let distance: isize = (&line[1..]).parse().unwrap();
        let direction = Direction::from_str(&line[0..1]).unwrap();
        ship.update_via_waypoint(direction, distance);
    }

    let Coordinate(x, y) = ship.position;

    let manhattan_dist = x.abs() + y.abs();
    println!("{}", manhattan_dist);
}

fn main() {
    let lines = read_lines("input.txt");
    part1(&lines);
    part2(&lines);
}
