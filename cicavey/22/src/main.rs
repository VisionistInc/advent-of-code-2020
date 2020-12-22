use aoc;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn part1(p1: &mut VecDeque<usize>, p2: &mut VecDeque<usize>) {
    loop {
        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();

        if c1 > c2 {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }


        if p1.is_empty() || p2.is_empty() {
            break;
        }
    }

    if p1.is_empty() {
        let score: usize = p2.iter().enumerate().map(|(i, v)| v * (p2.len() - i)).sum();
        println!("p2 {:?}", score);
    } else {
        let score: usize = p1.iter().enumerate().map(|(i, v)| v * (p1.len() - i)).sum();
        println!("p1 {:?}", score);
    }
}

fn game(g: usize, p1: &mut VecDeque<usize>, p2: &mut VecDeque<usize>) -> usize {

    // println!("=== Game {} === ", g);

    let mut prev_games: HashSet<u64> = HashSet::new();
    let mut r = 1;

// -- Round 1 (Game 1) --
// Player 1's deck: 9, 2, 6, 3, 1
// Player 2's deck: 5, 8, 4, 7, 10
// Player 1 plays: 9
// Player 2 plays: 5
// Player 1 wins round 1 of game 1!

    loop {

        // println!("\n-- Round {} (Game {}) --", r, g);

        // println!("Player 1's deck: {:?}", &p1);
        // println!("Player 2's deck: {:?}", &p2);

        // Calculate game state hash
        let mut hf = DefaultHasher::new();
        p1.hash(&mut hf);
        hf.write_usize(p1.len());
        p2.hash(&mut hf);
        hf.write_usize(p2.len());
        let hv = hf.finish();

        if !prev_games.insert(hv) {
            // game state same as prev, bail?
            // dbg!("duplicated state");

            if g == 1 {
                break;
            }

            return 1;
        }

        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();

        // println!("Player 1 plays: {}", c1);
        // println!("Player 2 plays: {}", c2);

        // which player is the winnder
        let w;

        // subgame?
        if p1.len() >= c1 && p2.len() >= c2 {
            let mut np1 = p1.clone();
            np1.truncate(c1);

            let mut np2 = p2.clone();
            np2.truncate(c2);

            w = game(g+1, &mut np1, &mut np2);
        } else if c1 > c2 {
            w = 1;
        } else {
            w = 2;
        }
        
        if w == 1 {
            p1.push_back(c1);
            p1.push_back(c2);
        } else { 
            p2.push_back(c2);
            p2.push_back(c1);
        }

        if p1.is_empty() {
            if g == 1 {
                break;
            }
            return 2;
        }

        if p2.is_empty() {
            if g == 1 {
                break;
            }
            return 1;
        }

        r += 1;
    }

    if p1.is_empty() {
        let score: usize = p2.iter().enumerate().map(|(i, v)| v * (p2.len() - i)).sum();
        println!("p2 {:?}", score);
    } else {
        let score: usize = p1.iter().enumerate().map(|(i, v)| v * (p1.len() - i)).sum();
        println!("p1 {:?}", score);
    }

    return 0;
}

fn part2(p1: &mut VecDeque<usize>, p2: &mut VecDeque<usize>) {
    game(1, &mut p1.clone(), &mut p2.clone());
}

fn main() {
    let lines = aoc::lines_from_file("input.txt");

    let mut p1: VecDeque<usize> = VecDeque::new();
    let mut p2: VecDeque<usize> = VecDeque::new();
    let mut d = &mut p1;

    for line in lines {
        if line == "Player 1:" {
            d = &mut p1;
            continue;
        }
        if line == "Player 2:" {
            d = &mut p2;
            continue;
        }
        if line.is_empty() {
            continue;
        }

        d.push_back(line.parse().unwrap())
    }

    part1(&mut p1.clone(), &mut p2.clone());
    part2(&mut p1.clone(), &mut p2.clone());
 
}
