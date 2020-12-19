use std::collections::HashMap;

// num_iters should be the total number of iterations, including the seed iterations
fn memory_game(num_iters: usize, seed: &Vec<usize>) {
    let mut prev_num: usize = 0; // this initial value will never be used
    let mut occurrences: HashMap<usize, usize> = HashMap::new();
    // start at 1 here because for the most part we'll be one-based
    // only zero-based thing here is indices of seed
    for i in 1..=num_iters {
        // first case
        if i == 0 {
            occurrences.insert(seed[i - 1], i);
            prev_num = seed[i - 1]
        } else {
            let curr_num = if (i as usize) <= seed.len() {
                seed[i - 1]
            } else {
                prev_num
            };

            let next_num = occurrences
                .get(&curr_num)
                .and_then(|last_occurrence| Some(i - last_occurrence))
                .unwrap_or(0);
            occurrences.insert(curr_num, i);

            if i == num_iters {
                // last number of the game
                println!("{}", prev_num);
            } else {
                prev_num = next_num;
            }
        }
    }
}

fn main() {
    // 0,6,1,7,2,19,20
    let input: Vec<usize> = "0,6,1,7,2,19,20"
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect();
    // memory_game(2020, &input); // part 1
    memory_game(30000000, &input); // part 2
}
