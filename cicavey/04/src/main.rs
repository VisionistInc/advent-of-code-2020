#[macro_use]
extern crate lazy_static;

use regex::Regex;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::collections::HashMap;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

// TODO why lifetimes

// byr (Birth Year) - four digits; at least 1920 and at most 2002.
fn byr<'a>(v: &'a &String) -> bool {
    match v.parse::<u16>() {
        Ok(n) => n >= 1920 && n <= 2002,
        _ => false,
    }
}

// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
fn iyr<'a>(v: &'a &String) -> bool {
    match v.parse::<u16>() {
        Ok(n) => n >= 2010 && n <= 2020,
        _ => false
    }
}

// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
fn eyr<'a>(v: &'a &String) -> bool {
    match v.parse::<u16>() {
        Ok(n) => n >= 2020 && n <= 2030,
        _ => false
    }
}

// hgt (Height) - a number followed by either cm or in:
// If cm, the number must be at least 150 and at most 193.
// If in, the number must be at least 59 and at most 76.
fn hgt<'a>(v: &'a &String) -> bool {

    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    }

    match RE.captures(v) {
        Some(caps) => {
            match caps.get(1).unwrap().as_str().parse::<i32>() {
                Ok(n) => {
                    match caps.get(2).unwrap().as_str() {
                        "cm" => n >= 150 && n <= 193,
                        "in" => n >= 59 && n <= 76,
                        _ => false
                    }
                },
                _ => false
            }
        },
        _ => false,
    }
}

// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
fn hcl<'a>(v: &'a &String) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    }
    RE.is_match(v)
}

// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
fn ecl<'a>(v: &'a &String) -> bool {
   match v.as_str() {
    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
    _ => false
   }
}

// pid (Passport ID) - a nine-digit number, including leading zeroes.
fn pid<'a>(v: &'a &String) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\d{9}$").unwrap();
    }
    RE.is_match(v)
}

fn valid(p : &HashMap<String, String>) -> Result<bool, &'static str> {
    if p.len() < 7  || (p.len() == 7 && p.contains_key("cid")) {
        return Err("Not enough fields")
    }

    // Validate each required field by criteria
    let byrv = p.get("byr").unwrap().as_str();
    match p.get("byr").filter(byr) {
        None => return Err("byr"),
        _ => ()
    }
    match p.get("iyr").filter(iyr) {
        None => return Err("iyr"),
        _ => ()
    }
    match p.get("eyr").filter(eyr) {
        None => return Err("eyr"),
        _ => ()
    }
    match p.get("hgt").filter(hgt) {
        None => return Err("hgt"),
        _ => ()
    }
    match p.get("hcl").filter(hcl) {
        None => return Err("hcl"),
        _ => ()
    }
    match p.get("ecl").filter(ecl) {
        None => return Err("ecl"),
        _ => ()
    }
    match p.get("pid").filter(pid) {
        None => return Err("pid"),
        _ => ()
    }
    Ok(true)
}

fn main() {
    let lines = lines_from_file("input.txt");

    // for each line, convert trees into points
    let mut data = Vec::new();
    let mut current: HashMap<String, String>  = HashMap::new();
    for line in lines {
        if line.is_empty() {
            data.push(current);
            current = HashMap::new();
            continue
        }
        for pair in line.split_whitespace() {
            let mut i = pair.splitn(2, ":");
            let key = i.next().unwrap();
            let value = i.next().unwrap();
            current.insert(key.to_string(), value.to_string());
        }
    }
    data.push(current);

    let v = data.iter().map(valid).filter(|&d| !d.is_err()).count();
    dbg!(v);
}
