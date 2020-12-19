use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;
use std::path::Path;

type Fields = HashMap<String, Vec<RangeInclusive<u32>>>;

struct Tickets {
    fields: Fields,
    your_ticket: Vec<u32>,
    nearby_tickets: Vec<Vec<u32>>,
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

fn parse(lines: &Vec<String>) -> Tickets {
    let mut lines = lines.iter();

    let mut fields: Fields = HashMap::new();
    // first, parse lines until we get to "your ticket"
    loop {
        if let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            // parse line (note the space to help with parsing)
            let field_and_val: Vec<&str> = line.split(": ").collect();
            let field_name = field_and_val[0].to_owned();
            // note more sneaky spaces!
            let ranges: Vec<&str> = field_and_val[1].split(" or ").collect();
            let mut parsed_ranges: Vec<RangeInclusive<u32>> = Vec::new();
            for range in ranges {
                let nums: Vec<u32> = range.split('-').map(|num| num.parse().unwrap()).collect();
                parsed_ranges.push(nums[0]..=nums[1])
            }
            fields.insert(field_name, parsed_ranges);
        } else {
            break;
        }
    }

    let mut lines = lines.skip(1);

    // parse your ticket
    let mut your_ticket: Vec<u32> = Vec::new();
    if let Some(line) = lines.next() {
        your_ticket = line.split(",").map(|num| num.parse().unwrap()).collect();
    }

    // parse the nearby tickets:
    let lines = lines.skip(2);
    let mut nearby_tickets: Vec<Vec<u32>> = Vec::new();
    for line in lines {
        let ticket: Vec<u32> = line.split(",").map(|num| num.parse().unwrap()).collect();
        nearby_tickets.push(ticket);
    }

    Tickets {
        fields,
        your_ticket,
        nearby_tickets,
    }
}

fn is_valid_ticket(ticket: &Vec<u32>, fields: &Fields) -> bool {
    'field: for ticket_field in ticket {
        for (_, valid_ranges) in fields {
            for range in valid_ranges {
                if range.contains(&ticket_field) {
                    // this value is valid, check the next value
                    continue 'field;
                }
            }
        }
        // this field was not in any valid range... so it's not valid as a ticket
        return false;
    }
    // should never get here
    return true;
}

fn part1(tickets: &Tickets) {
    let mut invalid_fields: Vec<u32> = Vec::new();
    // O(n^4) yay...
    for ticket in &tickets.nearby_tickets {
        // also with named breaks!
        'field: for ticket_field in ticket {
            for (_, valid_ranges) in &tickets.fields {
                for range in valid_ranges {
                    if range.contains(&ticket_field) {
                        continue 'field;
                    }
                }
            }
            // this field was not in any valid range... add to the invalid field list
            invalid_fields.push(*ticket_field)
        }
    }

    let error_rate: u32 = invalid_fields.iter().sum();
    println!("{}", error_rate);
}

fn find_required_matches(options: &HashMap<String, HashSet<usize>>) -> HashMap<String, usize> {
    let mut result: HashMap<String, usize> = HashMap::new();
    for (field, possible_positions) in options {
        let others: HashSet<usize> = options
            .iter()
            .filter_map(|(f, p)| if *f != *field { Some(p) } else { None })
            .fold(HashSet::new(), |acc, p| acc.union(p).map(|x| *x).collect());
        let diff: Vec<&usize> = possible_positions.difference(&others).collect();
        if !diff.is_empty() {
            result.insert(field.to_owned(), *diff[0]);
        }
        dbg!(field, possible_positions.difference(&others));
    }

    result
}

fn part2(tickets: &Tickets, keyword: &str) {
    // clone the nearby tickets in Tickets, filter out the invalid one, and collect
    let nearby_valid_tickets: Vec<&Vec<u32>> = tickets
        .nearby_tickets
        .iter()
        .filter(|ticket| is_valid_ticket(ticket, &tickets.fields))
        .collect();

    // keying on the location helps us with checking to avoid duplicates
    let mut possible_field_positions: HashMap<String, HashSet<usize>> = HashMap::new();

    // an even slower nested for loop... woo
    for field_name in tickets.fields.keys() {
        let mut possible_valid_positions: HashSet<usize> = HashSet::new();
        let ranges = tickets.fields.get(field_name.as_str()).unwrap();
        // check every field in a ticket
        'ticket_field: for i in 0..tickets.your_ticket.len() {
            // if all nearby tickets have this as a valid field, we know it's correct
            'ticket: for nearby_ticket in &nearby_valid_tickets {
                let val = nearby_ticket[i];
                for range in ranges {
                    if range.contains(&val) {
                        // this field is valid for this ticket, try the next one

                        continue 'ticket;
                    }
                }
                // if we're here, that means this field didn't work for this ticket, so try a new field
                continue 'ticket_field;
            }
            // if we made it here, this column fit the range for every nearby ticket, it's a possibility!
            possible_valid_positions.insert(i);
            // field_positions.insert(i, field_name.to_owned());
        }
        possible_field_positions.insert(field_name.to_owned(), possible_valid_positions);
    }
    // only get the fields that we are looking for
    let target_field_names: Vec<String> = tickets
        .fields
        .keys()
        .filter(|field_name| field_name.contains(keyword))
        .map(|field_name| field_name.to_owned())
        .collect();

    let mut final_positions: Vec<usize> = Vec::new();
    loop {
        // check for any fields that have a unique solution in this iteration
        for (field, position) in find_required_matches(&possible_field_positions) {
            // remove the fields with a solution from the list
            possible_field_positions.remove(&field);

            // remove the position of the field from the other options
            for (_, positions) in possible_field_positions.iter_mut() {
                positions.remove(&position);
            }

            // if this is a target field, add it
            if target_field_names.contains(&field) {
                final_positions.push(position);
            }
        }

        // once we get all the target fields, we're done!
        if final_positions.len() == target_field_names.len() {
            let result: u128 = final_positions
                .iter()
                .fold(1, |acc, i| acc * tickets.your_ticket[*i] as u128);
            println!("{}", result);
            return;
        }
    }
}

fn main() {
    let lines = read_lines("input.txt");
    let tickets = parse(&lines);
    part1(&tickets);
    part2(&tickets, "departure")
}
