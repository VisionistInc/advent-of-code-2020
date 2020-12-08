use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
use std::path::Path;

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

fn part1(answers: &Vec<String>) {
    let mut total_answers = 0;
    let mut group_answers: HashSet<char> = HashSet::new();

    for answer in answers {
        if answer.is_empty() {
            total_answers += group_answers.len();
            group_answers.drain();
        }

        group_answers.extend(answer.chars())
    }

    // one last time
    total_answers += group_answers.len();

    println!("{}", total_answers)
}

fn part2(answers: &Vec<String>) {
    let mut total_answers = 0;
    let mut group_answers = HashSet::new();

    // flag for skipping the rest of the group if there are no possible shared answers
    // fixes a bug with the group_answers.is_empty() check (would re-populate partway through group otherwise)
    let mut no_match = false;

    for answer in answers {
        if answer.is_empty() {
            total_answers += group_answers.len();

            // reset
            group_answers.drain();
            no_match = false;
        } else if !no_match {
            let individual_answers = HashSet::from_iter(answer.chars());

            if group_answers.is_empty() {
                // add in all individual answers if empty (initial case)
                group_answers.extend(individual_answers);
            } else {
                // only take answers that the group already has and this individual has
                group_answers = group_answers
                    .intersection(&individual_answers)
                    .map(|c| *c)
                    .collect();
                if group_answers.is_empty() {
                    no_match = true;
                }
            }
        }
    }

    // one last time
    total_answers += group_answers.len();

    println!("{}", total_answers)
}

fn main() {
    let lines = read_lines("input.txt");
    part1(&lines);
    part2(&lines);
}
