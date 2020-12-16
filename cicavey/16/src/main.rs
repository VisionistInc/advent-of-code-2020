use std::collections::HashSet;
use std::collections::HashMap;
use std::ops::RangeInclusive;
use regex::Regex;
use aoc;

type Rule = (String, RangeInclusive<u64>, RangeInclusive<u64>);

type Ticket = Vec<u64>;
type TicketSpace = Vec<HashSet<String>>;

fn match_rule(v: u64, r: &Rule) -> bool {
    r.1.contains(&v) || r.2.contains(&v)
}

fn guess(v: u64, rules: &Vec<Rule>) -> HashSet<String> {
    let mut potential: HashSet<String> = HashSet::new();

    for r in rules {
        if match_rule(v, &r) {
            potential.insert(r.0.clone());
        }
    }

    potential
}


fn expand(t: &Ticket, rules: &Vec<Rule>) -> TicketSpace {
    t.iter().map(|v| guess(*v, &rules)).collect()
}


#[derive(PartialEq)]
enum Mode {
    Rules,
    Your,
    Nearby,
}

fn main() {
    let lines = aoc::lines_from_file("input.txt");

    let p: Regex = Regex::new(r"(.*): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    let mut rules: Vec<Rule> = vec!();
    let mut your_ticket: Ticket = vec!();
    let mut nearby_tickets: Vec<Ticket> = vec!();

    let mut mode = Mode::Rules;

    for line in lines {

        if line.is_empty() {
            continue;
        }

        if line == "your ticket:" {
            mode = Mode::Your;
            continue;
        }

        if line == "nearby tickets:" {
            mode = Mode::Nearby;
            continue;
        }

        match &mode {
            Mode::Rules => {
                let caps = p.captures(&line).unwrap();
            
                let name = caps.get(1).unwrap().as_str().to_string();
                let min_a = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();
                let max_a = caps.get(3).unwrap().as_str().parse::<u64>().unwrap();
                let min_b = caps.get(4).unwrap().as_str().parse::<u64>().unwrap();
                let max_b = caps.get(5).unwrap().as_str().parse::<u64>().unwrap();
    
                rules.push((name, min_a..=max_a, min_b..=max_b))
            },
            Mode::Your => {
                your_ticket = line.split(",").map(|t| t.parse::<u64>().unwrap()).collect();
            },
            Mode::Nearby => {
                nearby_tickets.push(line.split(",").map(|t| t.parse::<u64>().unwrap()).collect());
            }
        }
    }

    let mut tickets: Vec<Ticket> = vec!();

    let mut error_rate: u64 = 0;
    for ticket in nearby_tickets {
        let mut valid = true;
        for v in &ticket {
            if !rules.iter().map(|r| match_rule(*v, r)).fold(false, |valid, cur| valid || cur) {
                error_rate += v;
                valid = false;
            }
        }

        if valid {
            tickets.push(ticket.clone());
        }
    }

    dbg!(error_rate);

    let yt_space: TicketSpace = expand(&your_ticket, &rules);

    // for each ticket, expand into all possible names for each field
    let tickets_space: Vec<TicketSpace> = tickets.iter().map(|t| expand(t, &rules)).collect();

    let ticket_size = tickets[0].len();

    // union all positional sets
    let mut merged_space: TicketSpace = vec!();
    for i in 0..ticket_size {

        let mut a = tickets_space[0][i].clone();

        for ts in &tickets_space {
            a = a.intersection(&ts[i]).map(|s| s.to_string()).collect();
        }

        merged_space.push(a);
    }

    // find "solved" constraints - sets with one value
    let mut resolved_fields : HashMap<usize, String> = HashMap::new();

    loop {
        let mut unresolved_count = 0;
        for (i, ts) in merged_space.iter_mut().enumerate() {
            if ts.len() == 1 {
                // record this, clear set
                resolved_fields.insert(i, ts.iter().next().unwrap().to_string());
                ts.clear();       
            }
            unresolved_count += ts.len();
        }

        // Remove all resolved fields
        for ts in merged_space.iter_mut() {
            for field in resolved_fields.values() {
                ts.remove(field);
            }
        }

        if unresolved_count == 0 {
            break;
        }
    }

    let mut dep_product: u64 = 1;
    for (i, v) in your_ticket.iter().enumerate() {
        let field = resolved_fields.get(&i).unwrap();
        if field.starts_with("departure") {
            dep_product *= v;
        }   
    }
    dbg!(dep_product);

}
