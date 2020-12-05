use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;
use std::path::Path;

// adapted from https://riptutorial.com/rust/example/4149/create-a-hashset-macro
macro_rules! set {
    ( $( $x:expr ),* ) => {  // Match zero or more comma delimited items
        {
            let mut temp_set = HashSet::new();  // Create a mutable HashSet
            $(
                temp_set.insert($x); // Insert each item matched into the HashSet
            )*
            temp_set // Return the populated HashSet
        }
    };
}

struct PassportValidator {
    byr: RangeInclusive<i32>,
    iyr: RangeInclusive<i32>,
    eyr: RangeInclusive<i32>,
    hgt_in: RangeInclusive<i32>,
    hgt_cm: RangeInclusive<i32>,
    hcl: Regex,
    ecl: Vec<&'static str>,
    pid: Regex,
}

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

fn parse_and_check_range(val: &str, range: &RangeInclusive<i32>) -> bool {
    if let Ok(val) = val.parse() {
        return range.contains(&val);
    }
    // parse didn't work... return false
    false
}

fn valid_passport_1(passport: &HashMap<String, String>, expected_fields: &HashSet<&str>) -> bool {
    let mut keys = HashSet::new();
    for key in passport.keys() {
        // slice the owned string fully to make it &str
        keys.insert(&key[..]);
    }

    // expected - keys because keys might have optional cid
    // this diff should be zero, since we're checking for things in expected_fields that are not in keys
    expected_fields.difference(&keys).count() == 0
}

fn valid_passport_2(passport: &HashMap<String, String>, expected_fields: &HashSet<&str>) -> bool {
    // constraint values are found on the website
    let constraints = PassportValidator {
        byr: (1920..=2002), // ..= means inclusive range
        iyr: (2010..=2020),
        eyr: (2020..=2030),
        hgt_in: (59..=76),
        hgt_cm: (150..=193),
        hcl: Regex::new(r"^#([0-9a-f]){6}$").unwrap(),
        ecl: vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"],
        pid: Regex::new(r"^([0-9]){9}$").unwrap(),
    };

    // first, make sure it has all the fields it needs
    // alternatively we could have collected the valid passports in part 1...
    if valid_passport_1(passport, expected_fields) {
        for (key, val) in passport {
            // this feels gross and i wish there was a better way to handle it
            match &key[..] {
                // TODO I feel like there should be a simpler way to do this. Maybe with optional?
                "byr" => {
                    if !parse_and_check_range(val, &constraints.byr) {
                        return false;
                    }
                }
                "iyr" => {
                    if !parse_and_check_range(val, &constraints.iyr) {
                        return false;
                    }
                }
                "eyr" => {
                    if !parse_and_check_range(val, &constraints.eyr) {
                        return false;
                    }
                }
                "hgt" => match &val[val.len() - 2..] {
                    // check for inches or cm
                    "in" => {
                        if !parse_and_check_range(&val[..val.len() - 2], &constraints.hgt_in) {
                            return false;
                        }
                    }
                    "cm" => {
                        if !parse_and_check_range(&val[..val.len() - 2], &constraints.hgt_cm) {
                            return false;
                        }
                    }
                    _ => return false,
                },
                "hcl" => {
                    if !constraints.hcl.is_match(val) {
                        return false;
                    }
                }
                "ecl" => {
                    if !constraints.ecl.contains(&&val[..]) {
                        return false;
                    }
                }
                "pid" => {
                    if !constraints.pid.is_match(val) {
                        return false;
                    }
                }
                _ => (), // ignore everything else
            }
        }
    } else {
        // not valid in part 1 so not valid here
        return false;
    }
    // fallthrough; must have been valid
    return true;
}

fn check_passports(
    lines: &Vec<String>,
    expected_fields: &HashSet<&str>,
    validate: fn(&HashMap<String, String>, &HashSet<&str>) -> bool,
) {
    let mut valid_passports = 0;
    let mut passport: HashMap<String, String> = HashMap::new();
    for line in lines {
        if line.is_empty() {
            if validate(&passport, expected_fields) {
                valid_passports += 1
            }

            // drain the passport for later use
            passport.drain();
        } else {
            for fields in line.split(' ') {
                let field: Vec<&str> = fields.split(':').collect();
                passport.insert(field[0].to_owned(), field[1].to_owned());
            }
        }
    }

    // check the passport one last time at the end since the iterator finishes before the last check
    if validate(&passport, expected_fields) {
        valid_passports += 1
    }

    println!("{}", valid_passports)
}

fn main() {
    // NOTE: "cid" is not expected but may be present!
    let expected_fields = set!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let lines = read_lines("input.txt");
    check_passports(&lines, &expected_fields, valid_passport_1);
    check_passports(&lines, &expected_fields, valid_passport_2)
}
