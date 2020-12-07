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

type Mappings = HashMap<String, HashMap<String, usize>>;

// Recurse until empty of child found
fn find(start: &String, query: &String, mappings: &Mappings) -> bool {

    // Get all of the children of start
    let children = mappings.get(start).unwrap();

    if children.len() == 0 {
        return false
    }

    let mut child_has_query = false;

    for child_key in children.keys() {
        if child_key == query {
            child_has_query = true;
            break;
        }
        child_has_query |= find(child_key, query, mappings);
    }

    return child_has_query
}

fn count(query: &String, mappings: &Mappings) -> usize {

    let children = mappings.get(query).unwrap();
    if children.len() == 0 {
        return 1;
    }

    // get each child
    let mut total = 0;
    for (child_key, child_count) in children {
        total += child_count * count(child_key, mappings);
    }

    return 1 + total;
}

fn main() {
    let re = Regex::new(r"\sbags?[,.]\s?").unwrap();

    let mut mappings: Mappings = HashMap::new();

    for line in lines_from_file("input.txt") {
        let v: Vec<&str> = line.split(" bags contain ").collect();

        let parent_bag_type = v.get(0).unwrap().to_string();
        let mut children = HashMap::new();

        for entry in re.split(v.get(1).unwrap()) {
            if entry.is_empty() { continue }

            if entry == "no other" {
                continue
            }

            let es = entry.to_string();
            let ev: Vec<&str> = es.splitn(2, ' ').collect();

            let bag_count = ev.get(0).unwrap().parse::<usize>().unwrap();
            let bag_type = ev.get(1).unwrap().to_string();

            children.insert(bag_type, bag_count);
        }

        mappings.insert(parent_bag_type, children);
    }

    let query = "shiny gold".to_string();

    // brute force. Check every node to see if it could eventually contain the query
    let parent_count = mappings.keys().filter(|key| find(key, &query, &mappings)).count();
    dbg!(parent_count);

    dbg!(count(&query, &mappings) - 1);
}
