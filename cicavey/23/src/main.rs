type Cups = Vec<i32>;

fn select_dest(cups: &Cups, start_label: i32) -> usize {
    let max_label = *cups.iter().max().unwrap();
    let min_label = *cups.iter().min().unwrap();
    let mut cur_label = start_label;
    cur_label -= 1;

    loop {
        match cups.iter().position(|v| *v == cur_label) {
            Some(idx) => {
                return idx
            }
            None => {
                cur_label -= 1;
                if cur_label < min_label {
                    cur_label = max_label;
                }
            }
        }
    }
}

fn index_of(cups: &Cups, value: i32) -> usize {
    cups.iter().position(|v| *v == value).unwrap()
}

fn part1() {
    let input = String::from("389125467"); // test
    //let input = String::from("872495136"); // input

    let mut cups: Cups = input.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();

    let mut cur = 0;

    for step in 1..=100 {

        // println!("-- move {} --", step);

        // print!("cups: ");

        // for (i, v) in cups.iter().enumerate() {
        //     if i == cur {
        //         print!("({}) ", v);
        //     } else {
        //         print!("{} ", v);
        //     }
        // }

        // println!();

        // grab the current label
        let cur_label = cups[cur];

        // remove three cups
        let mut rem = vec!();
        // indexes to delete
        let mut idxs = vec!();
        for i in 0..3 {
            let idx = (cur + 1 + i) % cups.len();
            idxs.push(idx);
            rem.push(cups[idx]);
        }
        idxs.sort();
        for idx in idxs.iter().rev() {
            cups.remove(*idx);
        }

        // println!("pickup: {:?}", &rem);

        let dest = select_dest(&cups, cur_label);

        // Select next cur by label
        let new_cur = index_of(&cups, cur_label);
        let next_label = cups[(new_cur + 1) % cups.len()];

        // println!("destination: {} (idx={}), cur = {}, new_cur={}, cur_label = {}, next_label = {} {:?}", cups[dest], &dest, cur, new_cur, cur_label, next_label, &cups);

        for r in rem.iter().rev() {
            cups.insert(dest+1, *r);
        }

        cur = index_of(&cups, next_label);

        // println!();
    }

    let start = index_of(&cups, 1);
    for i in 1..9 {
        print!("{}", cups[(start + i) % cups.len()]);
    }
    println!();
}

fn part2() {
    let input = String::from("389125467"); // test
    //let input = String::from("872495136"); // input

    let mut cups: Cups = input.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
 
    let mut new_cups: Cups = Vec::with_capacity(1_000_000);
    for v in &cups {
        new_cups.push(*v);
    }
    for v in 10..=1_000_000 {
        new_cups.push(v);
    }

    cups = new_cups;

    // println!("{} {} {}", new_cups[0], new_cups.last().unwrap(), new_cups.len());

    let mut cur = 0;

    for step in 1..=1000 {

        // println!("-- move {} --", step);

        // print!("cups: ");

        // for (i, v) in cups.iter().enumerate() {
        //     if i == cur {
        //         print!("({}) ", v);
        //     } else {
        //         print!("{} ", v);
        //     }
        // }

        // println!();

        // grab the current label
        let cur_label = cups[cur];

        // remove three cups
        let mut rem = vec!();
        // indexes to delete
        let mut idxs = vec!();
        for i in 0..3 {
            let idx = (cur + 1 + i) % cups.len();
            idxs.push(idx);
            rem.push(cups[idx]);
        }
        idxs.sort();
        for idx in idxs.iter().rev() {
            cups.remove(*idx);
        }

        // println!("pickup: {:?}", &rem);

        let dest = select_dest(&cups, cur_label);

        // Select next cur by label
        let new_cur = index_of(&cups, cur_label);
        let next_label = cups[(new_cur + 1) % cups.len()];

        // println!("destination: {} (idx={}), cur = {}, new_cur={}, cur_label = {}, next_label = {} {:?}", cups[dest], &dest, cur, new_cur, cur_label, next_label, &cups);

        for r in rem.iter().rev() {
            cups.insert(dest+1, *r);
        }

        cur = index_of(&cups, next_label);

        // println!();
    }

    let start = index_of(&cups, 1);
    for i in 1..9 {
        print!("{}", cups[(start + i) % cups.len()]);
    }
    println!();
}

fn main() {
    part1();
    part2();
}