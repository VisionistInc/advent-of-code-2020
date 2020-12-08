use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

type BagMap = HashMap<String, Option<HashMap<String, i32>>>;

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

fn parse_bag_rules(rules: &Vec<String>) -> BagMap {
    let mut bags: BagMap = HashMap::new();
    // check if there are extra bags and get this bag's name
    let re = Regex::new(r"(\w+ \w+) bags contain (no)?").unwrap();

    // get nested bag(s) and number
    let bag_re = Regex::new(r"(\d+) (\w+ \w+)").unwrap();
    for rule in rules {
        let caps = re.captures(rule).unwrap();
        let bag_name = caps.get(1).unwrap().as_str();

        // being "none" here means this bag doesn't nest!
        if caps.get(2).is_some() {
            bags.insert(bag_name.into(), None);
        } else {
            let mut nested_bags = HashMap::new();
            for nested_bag in rule.split(',') {
                let nested_caps = bag_re.captures(nested_bag).unwrap();
                let nested_num = nested_caps.get(1).unwrap().as_str();
                let nested_name = nested_caps.get(2).unwrap().as_str();
                nested_bags.insert(nested_name.into(), nested_num.parse().unwrap());
            }
            bags.insert(bag_name.into(), Some(nested_bags));
        }
    }
    bags
}

fn count_outer_bags(bags: &BagMap, expected_bag: &str, counted_bags: &mut HashSet<String>) {
    for (bag_name, others) in bags {
        if let Some(others) = others {
            if others.contains_key(expected_bag) {
                // add the found bag and recurse
                counted_bags.insert(bag_name.to_owned());

                count_outer_bags(bags, bag_name, counted_bags);
            }
        }
    }
}

fn part1(bags: &BagMap, expected_bag: &str) {
    let mut counted_bags = HashSet::new();
    count_outer_bags(bags, expected_bag, &mut counted_bags);
    println!("{}", counted_bags.len())
}

fn count_inner_bags(bags: &BagMap, expected_bag: &str) -> i32 {
    let mut inner_count = 1;
    // get() returns an optional, so we need to unwrap 2 optionals
    if let Some(Some(inner_bags)) = bags.get(expected_bag) {
        for (inner_bag, count) in inner_bags {
            // count this inner bag plus any bags inside of it
            inner_count += count * count_inner_bags(bags, inner_bag)
        }
    }

    inner_count
}

fn part2(bags: &BagMap, expected_bag: &str) {
    // this counts the starting bag as well, so we need to subtract one (I really should fix this...)
    let inner_bag_count = count_inner_bags(bags, expected_bag) - 1;
    println!("{}", inner_bag_count);
}

fn main() {
    let lines = read_lines("input.txt");
    let bags = parse_bag_rules(&lines);
    let expected_bag = "shiny gold";
    part1(&bags, expected_bag);
    part2(&bags, expected_bag);
}
